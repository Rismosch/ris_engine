use std::cell::RefCell;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::MutexGuard;
use std::sync::RwLock;
use std::sync::RwLockReadGuard;
use std::sync::RwLockWriteGuard;
use std::sync::TryLockError;
use std::thread;
use std::thread::JoinHandle;

use crate as ris_jobs;
use crate::errors::BlockedOrEmpty;
use crate::errors::IsEmpty;
use crate::job::Job;
use crate::job_buffer::JobBuffer;
use crate::job_future::JobFuture;
use crate::job_future::SettableJobFuture;

thread_local! {
    static WORKER_THREAD: RefCell<Option<WorkerThread>> = RefCell::new(None);
}

struct WorkerThread {
    local_buffer: Arc<JobBuffer>,
    steal_buffers: Vec<Arc<JobBuffer>>,
    index: usize,
}

pub struct JobSystemGuard {
    handles: Option<Vec<JoinHandle<()>>>,
    done: Arc<AtomicBool>,
}

impl Drop for JobSystemGuard {
    fn drop(&mut self) {
        ris_log::debug!("dropping job system...");

        self.done.store(true, Ordering::SeqCst);

        empty_buffer(0);

        match self.handles.take() {
            Some(handles) => {
                let mut i = 0;
                for handle in handles {
                    i += 1;
                    match handle.join() {
                        Ok(()) => ris_log::trace!("joined thread {}", i),
                        Err(_) => ris_log::fatal!("failed to join thread {}", i),
                    }
                }
            }
            None => ris_log::debug!("handles already joined"),
        }

        ris_log::info!("job system guard dropped!");
    }
}

pub const DEFAULT_BUFFER_CAPACITY: usize = 1024;

/// # Safety
///
/// The job system is a singleton. Initialize it only once.
pub unsafe fn init(
    buffer_capacity: usize,
    cpu_count: usize,
    threads: usize,
    set_affinity: bool,
) -> JobSystemGuard {
    // estimate workthreads and according affinities
    let threads = std::cmp::max(threads, 1);
    let threads = std::cmp::min(threads, cpu_count);

    let mut affinities = Vec::new();
    for _ in 0..threads {
        affinities.push(Vec::new());
    }

    for i in 0..cpu_count {
        affinities[i % threads].push(i);
    }

    // setup job buffers
    let mut buffers = Vec::with_capacity(threads);
    for _ in 0..threads {
        buffers.push(JobBuffer::new(buffer_capacity))
    }

    let done = Arc::new(AtomicBool::new(false));

    // setup worker threads
    let mut handles = Vec::with_capacity(threads - 1);
    for (i, core_ids) in affinities.iter().enumerate().take(threads).skip(1) {
        let core_ids = core_ids.clone();
        let buffers = duplicate_buffers(&buffers);
        let done_copy = done.clone();
        handles.push(thread::spawn(move || {
            setup_worker_thread(&core_ids, buffers, i, set_affinity);
            run_worker_thread(i, done_copy);
        }))
    }

    ris_log::debug!(
        "job system runs on {} threads. (1 main thread + {} additional threads)",
        handles.len() + 1,
        handles.len()
    );
    let handles = Some(handles);

    // setup main worker thread (this thread)
    let core_ids = affinities[0].clone();
    let buffers = duplicate_buffers(&buffers);
    setup_worker_thread(&core_ids, buffers, 0, set_affinity);

    JobSystemGuard { handles, done }
}

// public methods
pub fn submit<ReturnType: 'static, F: FnOnce() -> ReturnType + 'static>(
    job: F,
) -> JobFuture<ReturnType> {
    let (settable_future, future) = SettableJobFuture::new();

    let mut job = Job::new(move || {
        let result = job();
        settable_future.set(result);
    });

    loop {
        let not_pushed = WORKER_THREAD.with(|worker_thread| {
            if let Some(worker_thread) = worker_thread.borrow_mut().as_mut() {
                let push_result = unsafe { worker_thread.local_buffer.push(job) };
                match push_result {
                    Ok(()) => None,
                    Err(blocked_or_full) => {
                        Some(blocked_or_full.not_pushed)
                    }
                }
            } else {
                ris_log::error!("couldn't submit job, calling thread isn't a worker thread");
                None
            }
        });

        match not_pushed {
            Some(not_pushed) => job = not_pushed,
            None => break,
        }
    }

    future
}

pub fn run_pending_job(file: &str, line: u32) {
    match ris_jobs::job_system::pop_job(file, line) {
        Ok(mut job) => job.invoke(),
        Err(ris_jobs::errors::IsEmpty) => match ris_jobs::job_system::steal_job(file, line) {
            Ok(mut job) => job.invoke(),
            Err(ris_jobs::errors::BlockedOrEmpty) => std::thread::yield_now(),
        },
    }
}

pub fn lock<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    loop {
        match mutex.try_lock() {
            Ok(guard) => return guard,
            Err(TryLockError::WouldBlock) => {
                run_pending_job(file!(), line!());
            }
            Err(TryLockError::Poisoned(e)) => {
                ris_error::throw!("mutex is poisoned: {}", e);
            }
        }
    }
}

pub fn lock_read<T>(rw_lock: &RwLock<T>) -> RwLockReadGuard<T> {
    loop {
        match rw_lock.try_read() {
            Ok(guard) => return guard,
            Err(TryLockError::WouldBlock) => {
                run_pending_job(file!(), line!());
            }
            Err(TryLockError::Poisoned(e)) => {
                ris_error::throw!("mutex is poisoned: {}", e);
            }
        }
    }
}

pub fn lock_write<T>(rw_lock: &RwLock<T>) -> RwLockWriteGuard<T> {
    loop {
        match rw_lock.try_write() {
            Ok(guard) => return guard,
            Err(TryLockError::WouldBlock) => {
                run_pending_job(file!(), line!());
            }
            Err(TryLockError::Poisoned(e)) => {
                ris_error::throw!("mutex is poisoned: {}", e);
            }
        }
    }
}

pub fn thread_index() -> i32 {
    let mut result = -1;

    WORKER_THREAD.with(|worker_thread| {
        if let Some(worker_thread) = worker_thread.borrow().as_ref() {
            result = worker_thread.index as i32;
        } else {
            ris_log::error!("calling thread isn't a worker thread");
        }
    });

    result
}

// privat methods
fn duplicate_buffers(buffers: &Vec<Arc<JobBuffer>>) -> Vec<Arc<JobBuffer>> {
    let mut result = Vec::new();

    for buffer in buffers {
        result.push(buffer.clone());
    }

    result
}

fn setup_worker_thread(
    core_ids: &[usize],
    buffers: Vec<Arc<JobBuffer>>,
    index: usize,
    set_affinity: bool,
) {
    if set_affinity {
        match crate::affinity::set_affinity(core_ids) {
            Ok(()) => ris_log::trace!("set affinity {:?} for thread {}", core_ids, index),
            Err(error) => ris_log::error!("couldn't set affinity for thread {}: {}", index, error),
        };
    }

    let local_buffer = buffers[index].clone();
    let mut steal_buffers = Vec::new();

    for buffer in buffers.iter().skip(index + 1) {
        steal_buffers.push(buffer.clone());
    }

    for buffer in buffers.iter().take(index) {
        steal_buffers.push(buffer.clone());
    }

    WORKER_THREAD.with(move |worker_thread| {
        *worker_thread.borrow_mut() = Some(WorkerThread {
            local_buffer,
            steal_buffers,
            index,
        });
    });
}

fn run_worker_thread(index: usize, done: Arc<AtomicBool>) {
    while !done.load(Ordering::SeqCst) {
        run_pending_job(file!(), line!());
    }

    empty_buffer(index);
}

fn empty_buffer(index: usize) {
    loop {
        ris_log::trace!("emptying {}", index);
        match pop_job(file!(), line!()) {
            Ok(mut job) => job.invoke(),
            Err(IsEmpty) => break,
        }
    }
}

fn pop_job(file: &str, line: u32) -> Result<Job, IsEmpty> {
    let mut result = Err(IsEmpty);

    WORKER_THREAD.with(|worker_thread| {
        if let Some(worker_thread) = worker_thread.borrow_mut().as_mut() {
            result = unsafe { worker_thread.local_buffer.wait_and_pop() };
        } else {
            ris_log::error!(
                "couldn't pop job, calling thread isn't a worker thread. caller: {}:{}",
                file,
                line,
            );
        }
    });

    result
}

fn steal_job(file: &str, line: u32) -> Result<Job, BlockedOrEmpty> {
    let mut result = Err(BlockedOrEmpty);

    WORKER_THREAD.with(|worker_thread| {
        if let Some(worker_thread) = worker_thread.borrow_mut().as_mut() {
            for buffer in &worker_thread.steal_buffers {
                result = buffer.steal();
                if result.is_ok() {
                    break;
                }
            }
        } else {
            ris_log::error!(
                "couldn't steal job, calling thread isn't a worker thread. caller: {}:{}",
                file,
                line,
            );
        }
    });

    result
}

use std::{
    cell::RefCell,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex, MutexGuard,
    },
    thread::{self, JoinHandle},
};

use crate::{
    errors::{BlockedOrEmpty, IsEmpty},
    job::Job,
    job_buffer::JobBuffer,
    job_future::{JobFuture, SettableJobFuture},
};

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

pub fn init(buffer_capacity: usize, cpu_count: usize, threads: usize) -> JobSystemGuard {
    // estimate workthreads and according affinities
    let threads = std::cmp::min(cpu_count, threads);

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
            setup_worker_thread(&core_ids, buffers, i);
            run_worker_thread(i, done_copy);
        }))
    }

    ris_log::debug!("spawned {} additional worker threads", handles.len());
    let handles = Some(handles);

    // setup main worker thread (this thread)
    let core_ids = affinities[0].clone();
    let buffers = duplicate_buffers(&buffers);
    setup_worker_thread(&core_ids, buffers, 0);

    JobSystemGuard { handles, done }
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

        ris_log::debug!("job system finished")
    }
}

// public methods
pub fn submit<ReturnType: 'static, F: FnOnce() -> ReturnType + 'static>(
    job: F,
) -> JobFuture<ReturnType> {
    let mut not_pushed = None;

    let (settable_future, future) = SettableJobFuture::new();

    let job = Job::new(move || {
        let result = job();
        settable_future.set(result);
    });

    WORKER_THREAD.with(|worker_thread| {
        if let Some(worker_thread) = worker_thread.borrow_mut().as_mut() {
            let push_result = unsafe { worker_thread.local_buffer.push(job) };
            match push_result {
                Ok(()) => (),
                Err(blocked_or_full) => {
                    not_pushed = Some(blocked_or_full.not_pushed);
                }
            }
        } else {
            ris_log::error!("couldn't submit job, calling thread isn't a worker thread");
        }
    });

    if let Some(mut to_invoke) = not_pushed {
        to_invoke.invoke();
    }

    future
}

pub fn run_pending_job() {
    match pop_job() {
        Ok(mut job) => job.invoke(),
        Err(IsEmpty) => match steal_job() {
            Ok(mut job) => job.invoke(),
            Err(BlockedOrEmpty) => thread::yield_now(),
        },
    }
}

pub fn lock<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    loop {
        let try_lock_result = mutex.try_lock();

        if let Ok(mutex_guard) = try_lock_result {
            return mutex_guard;
        }

        run_pending_job();
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

fn setup_worker_thread(core_ids: &[usize], buffers: Vec<Arc<JobBuffer>>, index: usize) {
    match ris_os::affinity::set_affinity(core_ids) {
        Ok(()) => ris_log::trace!("set affinity {:?} for thread {}", core_ids, index),
        Err(error) => ris_log::error!("couldn't set affinity for thread {}: {}", index, error),
    };

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
        run_pending_job();
    }

    empty_buffer(index);
}

fn empty_buffer(index: usize) {
    loop {
        ris_log::trace!("emptying {}", index);
        match pop_job() {
            Ok(mut job) => job.invoke(),
            Err(IsEmpty) => break,
        }
    }
}

fn pop_job() -> Result<Job, IsEmpty> {
    let mut result = Err(IsEmpty);

    WORKER_THREAD.with(|worker_thread| {
        if let Some(worker_thread) = worker_thread.borrow_mut().as_mut() {
            result = unsafe { worker_thread.local_buffer.wait_and_pop() };
        } else {
            ris_log::error!("couldn't pop job, calling thread isn't a worker thread");
        }
    });

    result
}

fn steal_job() -> Result<Job, BlockedOrEmpty> {
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
            ris_log::error!("couldn't steal job, calling thread isn't a worker thread");
        }
    });

    result
}

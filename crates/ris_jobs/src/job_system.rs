use std::cell::RefCell;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::MutexGuard;
use std::thread;
use std::thread::JoinHandle;

use ris_util::ris_error::RisResult;

use crate::job::Job;
use crate::job_buffer::JobBuffer;
use crate::job_buffer::PopError;
use crate::job_buffer::PushError;
use crate::job_buffer::StealError;
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

/// # Safety
///
/// The JobSystem is a global system. Initialize it only once.
pub unsafe fn init(buffer_capacity: usize, cpu_count: usize, threads: usize) -> JobSystemGuard {
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
            let thread_result = run_worker_thread(i, done_copy);

            match thread_result {
                Ok(()) => ris_log::info!("job thread {} ended", i),
                Err(e) => ris_log::fatal!("job thread {} died: {}", i, e),
            }
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

        if let Err(e) = empty_buffer(0) {
            ris_log::fatal!("failed to empty main thread jobs: {}", e);
        }

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
) -> RisResult<JobFuture<ReturnType>> {
    let mut not_pushed = None;
    let mut ris_error = None;

    let (settable_future, future) = SettableJobFuture::new()?;

    let job = Job::new(move || {
        let result = job();
        let set_result = settable_future.set(result);
        if let Err(e) = set_result {
            ris_log::fatal!("failed to set future: {}", e);
        }
    });

    WORKER_THREAD.with(|worker_thread| {
        if let Some(worker_thread) = worker_thread.borrow_mut().as_mut() {
            let push_result = unsafe { worker_thread.local_buffer.push(job) };
            match push_result {
                Ok(()) => (),
                Err(error) => match error {
                    PushError::BlockedOrFull(j) => {
                        not_pushed = Some(j);
                    }
                    PushError::MutexPoisoned(p) => {
                        ris_error = Some(ris_util::new_err!(
                            "failed to push due to poisoned mutex: {}",
                            p
                        ));
                    }
                },
            }
        } else {
            ris_log::error!("couldn't submit job, calling thread isn't a worker thread");
        }
    });

    if let Some(r) = ris_error {
        return Err(r);
    }

    if let Some(mut to_invoke) = not_pushed {
        to_invoke.invoke();
    }

    Ok(future)
}

pub fn run_pending_job() -> RisResult<()> {
    let popped_job = pop_job()?;

    if let Some(mut job) = popped_job {
        job.invoke();
    } else {
        let stolen_job = steal_job()?;
        if let Some(mut job) = stolen_job {
            job.invoke();
        } else {
            thread::yield_now();
        }
    }

    Ok(())
}

pub fn lock<T>(mutex: &Mutex<T>) -> RisResult<MutexGuard<'_, T>> {
    loop {
        let try_lock_result = mutex.try_lock();

        if let Ok(mutex_guard) = try_lock_result {
            return Ok(mutex_guard);
        }

        run_pending_job()?;
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
    match crate::affinity::set_affinity(core_ids) {
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

fn run_worker_thread(index: usize, done: Arc<AtomicBool>) -> RisResult<()> {
    while !done.load(Ordering::SeqCst) {
        run_pending_job()?;
    }

    empty_buffer(index)
}

fn empty_buffer(index: usize) -> RisResult<()> {
    let mut counter = 0;
    ris_log::trace!("emptying {}...", index);
    while let Some(mut job) = pop_job()? {
        job.invoke();
        counter += 1;
    }

    ris_log::trace!("emptied {}! ({} jobs)", index, counter);
    Ok(())
}

fn pop_job() -> RisResult<Option<Job>> {
    let mut result = Ok(None);

    WORKER_THREAD.with(|worker_thread| {
        if let Some(worker_thread) = worker_thread.borrow_mut().as_mut() {
            let pop_result = unsafe { worker_thread.local_buffer.wait_and_pop() };
            match pop_result {
                Ok(job) => result = Ok(Some(job)),
                Err(e) => match e {
                    PopError::IsEmpty => (),
                    PopError::MutexPoisoned(p) => {
                        result =
                            ris_util::result_err!("failed to pop due to poisoned mutex: {}", p,);
                    }
                },
            }
        } else {
            ris_log::error!("couldn't pop job, calling thread isn't a worker thread");
        }
    });

    result
}

fn steal_job() -> RisResult<Option<Job>> {
    let mut result = Ok(None);

    WORKER_THREAD.with(|worker_thread| {
        if let Some(worker_thread) = worker_thread.borrow_mut().as_mut() {
            for buffer in &worker_thread.steal_buffers {
                let steal_result = buffer.steal();
                match steal_result {
                    Ok(job) => result = Ok(Some(job)),
                    Err(e) => match e {
                        StealError::BlockedOrEmpty => (),
                        StealError::TailPoisoned(p) => {
                            result = ris_util::result_err!(
                                "failed to steal due to poisoned mutex: {}",
                                p,
                            );
                        }
                        StealError::JobPoisoned(p) => {
                            result = ris_util::result_err!(
                                "failed to steal due to poisoned mutex: {}",
                                p,
                            );
                        }
                    },
                }
            }
        } else {
            ris_log::error!("couldn't steal job, calling thread isn't a worker thread");
        }
    });

    result
}

use std::{
    cell::RefCell,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    task::Poll,
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
    local_buffer: JobBuffer,
    steal_buffers: Vec<JobBuffer>,
    index: usize,
}

pub struct JobSystem {
    handles: Option<Vec<JoinHandle<()>>>,
    done: Arc<AtomicBool>,
}

impl JobSystem {
    pub fn new(buffer_capacity: usize, threads: usize) -> Self {
        let mut buffers = Vec::with_capacity(threads);
        for _ in 0..threads {
            buffers.push(JobBuffer::new(buffer_capacity))
        }

        let done = Arc::new(AtomicBool::new(false));

        let mut handles = Vec::with_capacity(threads);
        for i in 1..threads {
            let buffers = duplicate_buffers(&mut buffers);
            let done_copy = done.clone();
            handles.push(thread::spawn(move || {
                setup_worker_thread(i, buffers);
                run_worker_thread(i, done_copy);
            }))
        }

        // ris_log::info!("spawned {} worker threads", handles.len());
        let handles = Some(handles);

        let buffers = duplicate_buffers(&mut buffers);
        setup_worker_thread(0, buffers);

        Self { handles, done }
    }
}

impl Drop for JobSystem {
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

pub fn submit<ReturnType: Clone + 'static, F: FnOnce() -> ReturnType + 'static>(
    job: F,
) -> JobFuture<ReturnType> {
    let mut not_pushed = None;

    let (mut settable_future, future) = SettableJobFuture::new();

    let job = Job::new(move || {
        let result = job();
        settable_future.set(result);
    });

    WORKER_THREAD.with(|worker_thread| {
        if let Some(worker_thread) = worker_thread.borrow_mut().as_mut() {
            match worker_thread.local_buffer.push(job) {
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

pub fn wait<ReturnType: Clone>(future: JobFuture<ReturnType>) -> ReturnType {
    loop {
        match future.poll() {
            Poll::Pending => run_pending_job(),
            Poll::Ready(result) => {
                return result.clone();
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

fn duplicate_buffers(buffers: &mut Vec<JobBuffer>) -> Vec<JobBuffer> {
    let mut result = Vec::new();

    for buffer in buffers {
        result.push(buffer.duplicate());
    }

    result
}

fn setup_worker_thread(index: usize, buffers: Vec<JobBuffer>) {
    let mut buffers = buffers;

    let local_buffer = buffers[index].duplicate();
    let mut steal_buffers = Vec::new();
    for (i, steal_buffer) in buffers.iter_mut().enumerate() {
        if i == index {
            continue;
        }

        steal_buffers.push(steal_buffer.duplicate());
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

fn run_pending_job() {
    match pop_job() {
        Ok(mut job) => job.invoke(),
        Err(IsEmpty) => match steal_job() {
            Ok(mut job) => job.invoke(),
            Err(BlockedOrEmpty) => thread::yield_now(),
        },
    }
}

fn pop_job() -> Result<Job, IsEmpty> {
    let mut result = Err(IsEmpty);

    WORKER_THREAD.with(|worker_thread| {
        if let Some(worker_thread) = worker_thread.borrow_mut().as_mut() {
            result = worker_thread.local_buffer.wait_and_pop();
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
            for buffer in &mut worker_thread.steal_buffers {
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

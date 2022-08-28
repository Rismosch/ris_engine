use std::{sync::{atomic::{AtomicBool, Ordering}}, thread, cell::RefCell};

use crate::{job::Job, job_buffer::JobBuffer, errors::{IsEmpty, BlockedOrEmpty}};

struct WorkerThread {
    local_buffer: JobBuffer,
    steal_buffers: Vec<JobBuffer>,
    index: usize,
}

thread_local! {
    static WORKER_THREAD: RefCell<Option<WorkerThread>> = RefCell::new(None);
}

static DONE: AtomicBool = AtomicBool::new(false);

pub fn init_run_and_block(god_job: Job, buffer_capacity: usize){
    let cpus = num_cpus::get();
    ris_log::info!("cpu count: {}", cpus);

    DONE.store(false, Ordering::SeqCst);

    let mut buffers = Vec::with_capacity(cpus);
    for _ in 0..cpus {
        buffers.push(JobBuffer::new(buffer_capacity))
    }

    let wrapped_god_job = Job::new(move||{
        let mut god_job = god_job;
        god_job.invoke();
        DONE.store(true, Ordering::SeqCst);
    });
    
    match buffers[0].push(wrapped_god_job) {
        Ok(()) => {},
        Err(_) => ris_log::fatal!("god_job couldn't be submitted to the job system"),
    }
    
    let mut handles = Vec::with_capacity(cpus);
    for i in 1..cpus {
        let buffers = duplicate_buffers(&mut buffers);
        handles.push(thread::spawn(move || worker_thread(i, buffers)))
    }

    let buffers = duplicate_buffers(&mut buffers);
    worker_thread(0, buffers);

    let mut i = 0;
    for handle in handles {
        i += 1;
        match handle.join() {
            Ok(()) => ris_log::trace!("joined thread {}", i),
            Err(_) => ris_log::fatal!("failed to join thread {}", i),
        }
    }
}

fn duplicate_buffers(buffers: &mut Vec<JobBuffer>) -> Vec<JobBuffer> {
    let mut result = Vec::new();

    for buffer in buffers {
        result.push(buffer.duplicate());
    }

    result
}

// fn setup_worker_thread(index: usize, buffers: &mut Vec<JobBuffer>) {
//     let local_buffer = buffers[index].duplicate();
//     let mut steal_buffers = Vec::new();
//     for (i, buffer) in buffers.iter_mut().enumerate() {
//         if i == index {
//             continue;
//         }

//         let steal_buffer = buffer.duplicate();
//         steal_buffers.push(steal_buffer);
//     }

//     WORKER_THREAD.with(move |worker_thread|{
//         *worker_thread.borrow_mut() = Some(WorkerThread {
//             local_buffer,
//             steal_buffers,
//             index,
//         });
//     });
// }

pub fn submit<F: FnOnce() + 'static>(job: F){

    let mut not_pushed = None;

    WORKER_THREAD.with(|worker_thread|{
        if let Some(worker_thread) = worker_thread.borrow_mut().as_mut() {
            let job = Job::new(job);
    
            match worker_thread.local_buffer.push(job) {
                Ok(()) => (),
                Err(blocked_or_full) => {
                    not_pushed = Some(blocked_or_full.not_pushed);
                },
            }
        } else {
            ris_log::error!("couldn't submit job, calling thread isn't a worker thread");
        }
    });

    if let Some(mut to_invoke) = not_pushed {
        to_invoke.invoke();
    }
}

pub fn run_pending_job(){
    match pop_job() {
        Ok(job) => {
            let mut job = job;
            job.invoke();
        },
        Err(IsEmpty) => {
            match steal_job() {
                Ok(job) => {
                    let mut job = job;
                    job.invoke();
                }
                Err(BlockedOrEmpty) => thread::yield_now(),
            }
        },
    }
}

pub fn thread_index() -> usize {
    let mut result = 0;

    WORKER_THREAD.with(|worker_thread|{
        if let Some(worker_thread) = worker_thread.borrow().as_ref() {
            result = worker_thread.index;
        } else {
            ris_log::error!("calling thread isn't a worker thread");
            result = usize::MAX;
        }
    });

    result
}

fn worker_thread(index: usize, buffers: Vec<JobBuffer>) {
    let mut buffers = buffers;

    let local_buffer = buffers[index].duplicate();
    let mut steal_buffers = Vec::new();
    for i in 0..buffers.len() {
        if i == index {
            continue;
        }

        steal_buffers.push(buffers[i].duplicate());
    }

    WORKER_THREAD.with(move |worker_thread|{
        *worker_thread.borrow_mut() = Some(WorkerThread {
            local_buffer,
            steal_buffers,
            index,
        });
    });

    while !DONE.load(Ordering::SeqCst) {
        run_pending_job();
    }
}

fn pop_job() -> Result<Job, IsEmpty> {
    let mut result = Err(IsEmpty);

    WORKER_THREAD.with(|worker_thread|{
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

    WORKER_THREAD.with(|worker_thread|{
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
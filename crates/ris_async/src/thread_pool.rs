use std::cell::UnsafeCell;
use std::marker::PhantomData;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::MutexGuard;
use std::sync::TryLockError;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::thread::JoinHandle;
use std::thread::Thread;

use ris_error::prelude::*;

use crate::ris_deque::RisDeque;

//==============================================================================
// Utilities
//==============================================================================
const POISON_MESSAGE: &str = "mutex to not be poisoned";

pub struct Job(Box<dyn FnOnce() + Send>);

impl Job {
    pub fn from(callback: impl FnOnce() + Send + 'static) -> Self {
        Self(Box::new(callback))
    }

    pub fn invoke(self) {
        self.0()
    }
}

const BUFLEN: usize = 1<<10;
pub type JobDeque = Arc<Mutex<RisDeque<Job, BUFLEN>>>;
fn new_job_queue() -> JobDeque {
    Arc::new(Mutex::new(RisDeque::default()))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StallStrategy {
    SpinLoop,
    Yield,
    Parking,
}

//==============================================================================
// ThreadPool
//==============================================================================
pub struct ThreadPoolCreateInfo {
    pub cpu_count: usize,
    pub threads: usize,
    pub set_affinity: bool,
    pub stall_strategy: StallStrategy,
}

pub struct ThreadPool {
    join_handles: Vec<JoinHandle<()>>,
    boo: PhantomData<*const()>, // prohibit Send/Sync
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        ris_log::trace!("dropping thread_pool...");

        let worker = unsafe {self.main_worker()};
        worker.done.store(true, Ordering::Relaxed);

        while let Some(join_handle) = self.join_handles.pop() {
            let i = self.join_handles.len() + 1;
            join_handle.thread().unpark();
            match join_handle.join() {
                Ok(()) => ris_log::debug!("joined thread {}", i),
                Err(_) => ris_log::error!("failed to join thread {}", i),
            }
        }

        ThreadPoolWorker::finish(worker);
        ris_log::info!("dropped thread_pool!");
    }
}

impl ThreadPool {
    // public methods
    pub fn new(create_info: ThreadPoolCreateInfo) -> RisResult<Self> {
        if Self::worker().is_some() {
            return ris_error::new!("this thread is already part of a thread pool!");
        }

        let ThreadPoolCreateInfo {
            cpu_count,
            threads,
            set_affinity,
            stall_strategy,
        } = create_info;

        // prepare affinity
        let affinities = if !set_affinity {
            None
        } else {
            let threads = std::cmp::max(threads, 1);
            let threads = std::cmp::min(threads, cpu_count);

            let mut affinities = Vec::with_capacity(threads);
            for _ in 0..threads {
                affinities.push(Vec::new());
            }

            for i in 0..cpu_count {
                affinities[i % threads].push(i);
            }

            Some(affinities)
        };

        // setup worker threads
        let worker_create_info = ThreadPoolWorkerCreateInfo {
            done: Arc::new(AtomicBool::new(false)),
            id: 0,
            affinities,
            pool_queue: new_job_queue(),
            all_queues: vec![new_job_queue(); threads],
            worker_threads: Arc::new(Mutex::new(None)),
            stall_strategy,
        };

        let mut join_handles = Vec::with_capacity(threads - 1);
        for id in 1..threads {
            let mut worker_create_info = worker_create_info.clone();
            worker_create_info.id = id;

            let join_handle = std::thread::Builder::new()
                .name(format!("thread_pool_worker_{}", id))
                .spawn(move || {
                    let worker = ThreadPoolWorker::new(worker_create_info);
                    worker.run();
                    worker.finish();
                })?;
            join_handles.push(join_handle);
        }

        // collect worker threads
        let mut worker_threads = Vec::with_capacity(join_handles.len());
        for join_handle in join_handles.iter() {
            let thread = join_handle.thread().clone();
            worker_threads.push(thread);
        }

        let mut g = worker_create_info.worker_threads.lock()
            .ris_pexpect(POISON_MESSAGE);
        *g = Some(worker_threads.clone());
        drop(g);
        for worker_thread in worker_threads {
            worker_thread.unpark();
        }

        // setup main worker
        ThreadPoolWorker::new(worker_create_info);

        Ok(ThreadPool {
            join_handles,
            boo: Default::default()
        })
    }

    pub fn submit(&mut self, job: impl FnOnce() + Send + 'static) {
        let worker = unsafe {self.main_worker()};
        worker.submit_to_queue(job, &worker.pool_queue);
    }

    pub fn worker() -> Option<&'static ThreadPoolWorker> {
        get_worker()
    }

    /// # Safety
    ///
    /// returned worker MUST NOT live longer than self
    pub unsafe fn main_worker(&mut self) -> &'static ThreadPoolWorker {
        let Some(worker) = Self::worker() else {
            // enforced by the function signature, which takes a `&mut self`
            unreachable!("main thread should always be a worker"); 
        };

        worker
    }
}


//==============================================================================
// Worker
//==============================================================================
thread_local! {
    static WORKER: UnsafeCell<Option<ThreadPoolWorker>> = const {
        UnsafeCell::new(None)
    };
}

fn set_worker(value: Option<ThreadPoolWorker>) {
    WORKER.with(|worker| {
        *unsafe { &mut *worker.get() } = value;
    })
}

fn get_worker() -> Option<&'static ThreadPoolWorker> {
    WORKER.with(|worker| unsafe { &*worker.get() }.as_ref())
}

pub struct ThreadPoolWorker {
    done: Arc<AtomicBool>,
    local_queue: JobDeque,
    pool_queue: JobDeque,
    other_queues: Vec<JobDeque>,
    worker_threads: Vec<Thread>,
    stall_strategy: StallStrategy,
}

#[derive(Clone)]
struct ThreadPoolWorkerCreateInfo {
    done: Arc<AtomicBool>,
    id: usize,
    affinities: Option<Vec<Vec<usize>>>,
    pool_queue: JobDeque,
    all_queues: Vec<JobDeque>,
    worker_threads: Arc<Mutex<Option<Vec<Thread>>>>,
    stall_strategy: StallStrategy,
}

impl ThreadPoolWorker {
    // construction
    fn new(create_info: ThreadPoolWorkerCreateInfo) -> &'static Self {
        let ThreadPoolWorkerCreateInfo {
            done,
            id,
            affinities,
            pool_queue,
            all_queues,
            worker_threads,
            stall_strategy,
        } = create_info;

        // set affinity
        if let Some(affinities) = affinities {
            ris_error::panic_assert!(id < affinities.len());

            let core_ids = &affinities[id];
            let result = crate::affinity::set_affinity(core_ids);
            if let Err(e) = result {
                ris_log::error!("failed to set affinities for worker {}: {}", id, e);
            }
        }

        // setup queues
        ris_error::panic_assert!(id < all_queues.len());

        let local_queue = all_queues[id].clone();
        let mut other_queues = Vec::with_capacity(all_queues.len() - 1);

        for i in 1..all_queues.len() {
            let index = (i + id) % all_queues.len();
            let queue = all_queues[index].clone();
            other_queues.push(queue);
        }

        // setup worker threads
        loop {
            let g = worker_threads
                .lock()
                .ris_pexpect(POISON_MESSAGE);
            if g.is_some() {
                break;
            }

            drop(g);
            std::thread::park();
        }

        let worker_threads = worker_threads
            .lock()
            .ris_pexpect(POISON_MESSAGE)
            .clone()
            .ris_pexpect("worker threads to be set at this point");

        // set thread local variabel
        set_worker(Some(Self {
            done,
            local_queue,
            pool_queue,
            other_queues,
            worker_threads,
            stall_strategy,
        }));
        get_worker().ris_pexpect("worker to be set up")
    }

    fn run(&self) {
        ris_log::trace!("thread_pool running... \"{}\"", self.name());

        while !self.done.load(Ordering::Relaxed) {
            let job_was_run = self.run_pending_job();
            if job_was_run {
                continue;
            }

            match self.stall_strategy {
                StallStrategy::SpinLoop => std::hint::spin_loop(),
                StallStrategy::Yield => std::thread::yield_now(),
                StallStrategy::Parking => std::thread::park(),
            }
        }

        ris_log::trace!("thread_pool exited \"{}\"", self.name());
    }

    fn finish(&self) {
        let name = self.name();
        ris_log::trace!("thread_pool finishing... \"{}\"", name);
        while self.run_pending_job() {}
        set_worker(None);
        ris_log::debug!("thread_pool finished \"{}\"!", name);
    }

    // public methods
    pub fn name(&self) -> String {
        let current = std::thread::current();
        let name = std::thread::Thread::name(&current);
        name.map(|x| x.to_string()).unwrap_or("no name".to_string())
    }

    pub fn submit(&self, job: impl FnOnce() + Send + 'static) {
        self.submit_to_queue(job, &self.local_queue);
    }

    pub fn run_pending_job(&self) -> bool {
        match self.get_job() {
            Some(job) => {
                job.invoke();
                true
            },
            None => false,
        }
    }

    pub fn lock<'a, T>(&'a self, mutex: &'a Mutex<T>) -> MutexGuard<'a, T> {
        loop {
            match mutex.try_lock() {
                Ok(guard) => return guard,
                Err(TryLockError::WouldBlock) => {
                    let job_was_run = self.run_pending_job();
                    if !job_was_run {
                        std::hint::spin_loop();
                    }
                }
                Err(TryLockError::Poisoned(e)) => {
                    ris_error::panic!("mutex is poisoned: {}", e);
                }
            }
        }
    }

    // private methods
    fn submit_to_queue(&self, job: impl FnOnce() + Send + 'static, queue: &JobDeque) {
        let mut queue = self.lock(&*queue);

        let mut to_enque: Result<(), Job> = Err(Job::from(job));
        loop {
            let Err(job) = to_enque else {
                break;
            };

            to_enque = queue.push_back(job);
        }

        for thread in self.worker_threads.iter() {
            thread.unpark();
        }
    }

    fn get_job(&self) -> Option<Job> {
        // get local job
        let local_job = self.local_queue
            .lock()
            .ris_pexpect(POISON_MESSAGE)
            .pop_back();

        if let Some(job) = local_job {
            return Some(job)
        }

        // get pool job
        let pool_job = self.pool_queue
            .lock()
            .ris_pexpect(POISON_MESSAGE)
            .pop_back();

        if let Some(job) = pool_job {
            return Some(job)
        }

        // steal job
        for other in self.other_queues.iter() {
            let stolen_job = other
                .lock()
                .ris_pexpect(POISON_MESSAGE)
                .pop_front();

            if let Some(job) = stolen_job {
                return Some(job)
            }
        }

        // everyone is empty!
        None
    }
}


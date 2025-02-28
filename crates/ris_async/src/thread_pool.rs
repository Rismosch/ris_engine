use std::cell::UnsafeCell;
use std::future::Future;
use std::marker::PhantomData;
use std::pin::pin;
use std::pin::Pin;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::MutexGuard;
use std::sync::TryLockError;
use std::task::Context;
use std::task::Poll;
use std::task::Wake;
use std::thread::JoinHandle;
use std::thread::Thread;

use ris_error::Extensions;
use ris_error::RisResult;

use crate::JobFuture;
use crate::Receiver;
use crate::Sender;
use crate::SpinLock;
use crate::Stealer;

type Job = Box<dyn Future<Output = ()>>;

#[derive(Clone)]
struct ThreadWaker(Option<Thread>);

impl Wake for ThreadWaker {
    fn wake(self: Arc<Self>) {
        if let Some(thread) = &self.0 {
            thread.unpark();
        }
    }
}

thread_local! {
    static WORKER: UnsafeCell<Option<Worker>> = const { UnsafeCell::new(None)};
}

fn set_worker(value: Option<Worker>) {
    WORKER.with(|worker| {
        *unsafe { &mut *worker.get() } = value;
    })
}

fn get_worker() -> Option<&'static Worker> {
    WORKER.with(|worker| unsafe { &*worker.get() }.as_ref())
}

pub struct Worker {
    done: Arc<AtomicBool>,
    sender: Sender<Job>,
    receiver: Receiver<Job>,
    waker: ThreadWaker,
    others: Vec<OtherWorker>,
    use_parking: bool,
}

pub struct OtherWorker {
    stealer: Arc<Stealer<Job>>,
    waker: ThreadWaker,
}

impl Worker {
    fn name(&self) -> String {
        let current = std::thread::current();
        let name = std::thread::Thread::name(&current);
        name.map(|x| x.to_string()).unwrap_or("no name".to_string())
    }

    fn run(&self) {
        while !self.done.load(Ordering::Relaxed) {
            if !self.run_pending_job() {
                if self.use_parking {
                    std::thread::park();
                } else {
                    std::hint::spin_loop();
                }
            }
        }
    }

    fn run_pending_job(&self) -> bool {
        match self.get_job() {
            Some(job) => {
                let pinned = Pin::from(job);
                self.block_on(pinned);
                true
            }
            None => false,
        }
    }

    fn get_job(&self) -> Option<Job> {
        match self.receiver.receive() {
            Some(job) => Some(job),
            None => {
                for other in self.others.iter() {
                    if let Some(job) = other.stealer.steal() {
                        return Some(job);
                    };
                }

                None
            }
        }
    }

    fn block_on<F: Future>(&self, future: F) -> F::Output {
        let mut pinned_future = pin!(future);

        let waker = Arc::new(self.waker.clone()).into();
        let mut context = Context::from_waker(&waker);

        loop {
            match pinned_future.as_mut().poll(&mut context) {
                Poll::Ready(result) => return result,
                Poll::Pending => {
                    if !self.run_pending_job() {
                        std::thread::yield_now();
                    }
                }
            }
        }
    }

    fn wake_other_workers(&self) {
        for other in self.others.iter() {
            Arc::new(other.waker.clone()).wake();
        }
    }
}

pub struct ThreadPoolCreateInfo {
    pub buffer_capacity: usize,
    pub cpu_count: usize,
    pub threads: usize,
    pub set_affinity: bool,
    pub use_parking: bool,
}

pub struct ThreadPool {
    _boo: PhantomData<()>, // prevent ThreadPool to be instantiated
}

pub struct ThreadPoolGuard {
    done: Arc<AtomicBool>,
    join_handles: Option<Vec<JoinHandle<()>>>,
}

impl Drop for ThreadPoolGuard {
    fn drop(&mut self) {
        ris_log::trace!("dropping thread_pool...");

        self.done.store(true, Ordering::Relaxed);

        match get_worker() {
            Some(worker) => {
                worker.wake_other_workers();
                while worker.run_pending_job() {}
                set_worker(None);
            }
            None => {
                ris_log::error!("thread pool was dropped on a non worker thread")
            }
        }

        match self.join_handles.take() {
            Some(join_handles) => {
                for (i, join_handle) in join_handles.into_iter().enumerate() {
                    let i = i + 1;
                    match join_handle.join() {
                        Ok(()) => ris_log::debug!("joined thread {}", i),
                        Err(_) => ris_log::error!("failed to join thread {}", i),
                    }
                }
            }
            None => ris_log::error!("no handles to join"),
        }

        ris_log::info!("dropped thread_pool!");
    }
}

impl ThreadPool {
    pub fn init(create_info: ThreadPoolCreateInfo) -> RisResult<ThreadPoolGuard> {
        let ThreadPoolCreateInfo {
            buffer_capacity,
            cpu_count,
            threads,
            set_affinity,
            use_parking,
        } = create_info;

        let threads = std::cmp::max(threads, 1);
        let threads = std::cmp::min(threads, cpu_count);

        let mut affinities = Vec::with_capacity(threads);
        for _ in 0..affinities.capacity() {
            affinities.push(Vec::new());
        }

        for i in 0..cpu_count {
            affinities[i % threads].push(i);
        }

        // setup shared worker data
        let initial_worker_data = Arc::new(SpinLock::new(Vec::new()));
        let prepared_worker_data = Arc::new(SpinLock::new(Vec::<Option<_>>::new()));
        let done_preparing_worker_data = Arc::new(AtomicBool::new(false));

        let mut g = initial_worker_data.lock();
        for _ in 0..threads {
            g.push(None);
        }
        drop(g);

        // initial main worker setup
        if set_affinity {
            if let Err(e) = crate::affinity::set_affinity(&affinities[0]) {
                ris_log::error!("failed to set affinities for main worker: {}", e);
            }
        }
        let (sender, receiver, stealer) = crate::channel::<Job>(buffer_capacity);
        let waker = if use_parking {
            ThreadWaker(Some(std::thread::current()))
        } else {
            ThreadWaker(None)
        };
        initial_worker_data.lock()[0] = Some(OtherWorker {
            stealer,
            waker: waker.clone(),
        });

        // setup worker threads
        let done = Arc::new(AtomicBool::new(false));

        let mut join_handles = Vec::with_capacity(threads - 1);
        for (i, core_ids) in affinities.iter().enumerate().take(threads).skip(1) {
            let core_ids = core_ids.clone();
            let done = done.clone();
            let initial_worker_data = initial_worker_data.clone();
            let prepared_worker_data = prepared_worker_data.clone();
            let done_preparing_worker_data = done_preparing_worker_data.clone();

            let join_handle = std::thread::Builder::new()
                .name(format!("thread_pool.worker.{}", i))
                .spawn(move || {
                    // worker initial setup
                    if set_affinity {
                        if let Err(e) = crate::affinity::set_affinity(&core_ids) {
                            ris_log::error!("failed to set affinities for worker {}: {}", i, e);
                        }
                    }
                    let (sender, receiver, stealer) = crate::channel::<Job>(buffer_capacity);
                    let waker = if use_parking {
                        ThreadWaker(Some(std::thread::current()))
                    } else {
                        ThreadWaker(None)
                    };
                    initial_worker_data.lock()[i] = Some(OtherWorker {
                        stealer,
                        waker: waker.clone(),
                    });

                    while !done_preparing_worker_data.load(Ordering::Relaxed) {
                        std::thread::yield_now();
                    }

                    // prepare worker
                    let mut g = prepared_worker_data.lock();
                    let others = ris_error::unwrap!(
                        g[i].take().into_ris_error(),
                        "something has gone terribly wrong. this option should never be none"
                    );
                    drop(g);

                    set_worker(Some(Worker {
                        done,
                        sender,
                        receiver,
                        waker,
                        others,
                        use_parking,
                    }));

                    let worker = ris_error::unwrap!(
                        get_worker().into_ris_error(),
                        "something has gone terribly wrong. this option should never be none"
                    );

                    // run worker
                    let worker_name = worker.name();
                    ris_log::trace!("thread_pool running \"{}\"", worker_name);
                    worker.run();

                    ris_log::trace!("thread_pool finishing... \"{}\"", worker_name);
                    while worker.run_pending_job() {}
                    set_worker(None);
                    ris_log::debug!("thread_pool finished \"{}\"!", worker_name);
                })?;
            join_handles.push(join_handle);
        }

        // wait until all workers have shared their initial worker data
        for i in 0..threads {
            while initial_worker_data.lock()[i].is_none() {
                std::thread::yield_now();
            }
        }

        // all workers are initialized, prepare worker data for everyone
        for i in 0..threads {
            let mut shared = Vec::new();
            let g = initial_worker_data.lock();
            for j in 0..g.len() {
                // offset j, such that every worker has a different stealer at first, this attempts
                // to reduce contention
                let j = (j + i) % threads;
                if i == j {
                    continue;
                }

                let original = &g[j];
                let other = ris_error::unwrap!(
                    original.as_ref().into_ris_error(),
                    "something has gone terribly wrong. this option should never be none"
                );

                shared.push(OtherWorker {
                    stealer: other.stealer.clone(),
                    waker: other.waker.clone(),
                });
            }
            drop(g);
            prepared_worker_data.lock().push(Some(shared));
        }

        done_preparing_worker_data.store(true, Ordering::Relaxed);

        // prepare main worker
        let mut g = prepared_worker_data.lock();
        let others = ris_error::unwrap!(
            g[0].take().into_ris_error(),
            "something has gone terribly wrong. this option should never be none"
        );
        drop(g);

        set_worker(Some(Worker {
            done: done.clone(),
            sender,
            receiver,
            waker,
            others,
            use_parking,
        }));

        // return thread pool
        Ok(ThreadPoolGuard {
            done,
            join_handles: Some(join_handles),
        })
    }

    pub fn submit<F: Future + 'static>(future: F) -> JobFuture<F::Output> {
        let Some(worker) = get_worker() else {
            ris_error::throw!("cannot submit future, caller is not a worker");
        };

        let (job_future, job_future_setter) = JobFuture::new();

        let mut job: Box<dyn Future<Output = ()>> = Box::new(async move {
            let output = future.await;
            job_future_setter.set(output);
        });

        loop {
            match worker.sender.send(job) {
                Ok(()) => break,
                Err(not_sent) => {
                    if !worker.run_pending_job() {
                        std::thread::yield_now();
                    }
                    job = not_sent;
                }
            }
        }

        worker.wake_other_workers();

        job_future
    }

    pub fn block_on<F: Future>(future: F) -> F::Output {
        let Some(worker) = get_worker() else {
            ris_error::throw!("cannot block on future, caller is not a worker");
        };

        worker.block_on(future)
    }

    pub fn run_pending_job() -> bool {
        let Some(worker) = get_worker() else {
            ris_log::error!("cannot run pending job, caller is not a worker");
            return false;
        };

        worker.run_pending_job()
    }

    pub fn lock<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
        loop {
            match mutex.try_lock() {
                Ok(guard) => return guard,
                Err(TryLockError::WouldBlock) => {
                    Self::run_pending_job();
                }
                Err(TryLockError::Poisoned(e)) => {
                    ris_error::throw!("mutex is poisoned: {}", e);
                }
            }
        }
    }
}

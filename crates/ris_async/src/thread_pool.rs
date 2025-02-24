use std::cell::RefCell;
use std::cell::UnsafeCell;
use std::marker::PhantomData;
use std::pin::Pin;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::future::Future;
use std::task::Wake;
use std::thread::JoinHandle;
use std::thread::Thread;

use ris_error::Extensions;
use ris_error::RisResult;

use crate::Channel;
use crate::Sender;
use crate::Stealer;
use crate::Receiver;
use crate::SpinLock;

pub const DEFAULT_BUFFER_CAPACITY: usize = 1024;

type Job = Box<dyn Future<Output = ()>>;

struct ThreadWaker(Thread);

impl Wake for ThreadWaker {
    fn wake(self: Arc<Self>) {
        self.0.unpark();
    }
}

thread_local! {
    static WORKER: UnsafeCell<Option<Worker>> = const { UnsafeCell::new(None)};
}

fn set_worker(value: Worker) {
    WORKER.with(|worker| {
        *unsafe{&mut *worker.get()} = Some(value);
    })
}

fn get_worker() -> Option<&'static Worker> {
    WORKER.with(|worker| {
        unsafe {&*worker.get()}.as_ref()
    })
}

pub struct Worker {
    done: Arc<AtomicBool>,
    sender: Sender<Job>,
    receiver: Receiver<Job>,
    shared: Vec<SharedWorkerData>,
}

pub struct SharedWorkerData {
    stealer: Arc<Stealer<Job>>,
    waker: Arc<ThreadWaker>,
}

impl Worker {
    fn name(&self) -> String {
        let current = std::thread::current();
        let name = std::thread::Thread::name(&current);
        name.map(|x| x.to_string()).unwrap_or("no name".to_string())
    }

    fn run(&self) {
        println!("running \"{}\"", self.name());

        while !self.done.load(Ordering::Relaxed) {
            if !self.run_pending_job() {
                std::thread::park();
            }
        }

        println!("finishing... \"{}\"", self.name());
        while self.run_pending_job() {}
        println!("finished \"{}\"!", self.name());
    }

    fn run_pending_job(&self) -> bool {
        match self.get_job() {
            Some(job) => {
                let pinned = Pin::from(job);
                self.block_on(pinned);
                true
            },
            None => false,
        }
    }

    fn get_job(&self) -> Option<Job> {
        match self.receiver.receive() {
            Some(job) => Some(job),
            None => {
                for SharedWorkerData { stealer, waker } in self.shared.iter() {
                    if let Some(job) = stealer.steal() {
                        return Some(job)
                    };
                }

                None
            }
        }
    }

    fn block_on<F: Future>(&self, future: F) -> F::Output {
        panic!();
    }

    fn wake_other_workers(&self) {
        for SharedWorkerData { stealer, waker } in self.shared.iter() {
            waker.clone().wake();
        }
    }
}

pub struct ThreadPool {
    done: Arc<AtomicBool>,
    join_handles: Option<Vec<JoinHandle<()>>>,
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("dropping thread pool...");

        self.done.store(true, Ordering::Relaxed);

        match get_worker() {
            Some(worker) => {
                worker.wake_other_workers();
                while worker.run_pending_job() {}
            },
            None => {
                println!("thread pool was dropped on a non worker thread")
            }
        }

        match self.join_handles.take() {
            Some(join_handles) => {
                for (i, join_handle) in join_handles.into_iter().enumerate() {
                    let i = i + 1;
                    match join_handle.join() {
                        Ok(()) => println!("joined thread {}", i),
                        Err(_) => println!("failed to join thread {}", i),
                    }
                }
            },
            None => println!("no handles to join"),
        }

        println!("dropped thread pool!");
    }
}

impl ThreadPool {
    pub fn new(
        buffer_capacity: usize,
        cpu_count: usize,
        threads: usize,
        set_affinity: bool,
    ) -> RisResult<Self> {
        let threads = std::cmp::max(threads, 1);
        let threads = std::cmp::min(threads, cpu_count);

        let mut affinities = Vec::with_capacity(threads);
        for _ in 0..affinities.capacity() {
            affinities.push(Vec::new());
        }

        for i in 0..cpu_count {
            affinities[i % threads].push(i);
        }

        // setup stealers
        let original_worker_data = Arc::new(SpinLock::new(Vec::new()));
        let shared_worker_data = Arc::new(SpinLock::new(Vec::<Option<_>>::new()));
        let done_preparing_shared_worker_data = Arc::new(AtomicBool::new(false));

        let mut g = original_worker_data.lock();
        for _ in 0..threads {
            g.push(None);
        }
        drop(g);

        // initial main worker setup
        crate::affinity::set_affinity(&affinities[0]);
        let (
            sender,
            receiver,
            stealer,
        ) = Channel::<Job>::new(buffer_capacity);
        let waker = Arc::new(ThreadWaker(std::thread::current()));
        original_worker_data.lock()[0] = Some((stealer, waker));

        // setup worker threads
        let done = Arc::new(AtomicBool::new(false));
        
        let mut join_handles = Vec::with_capacity(threads - 1);
        for (i, core_ids) in affinities.iter().enumerate().take(threads).skip(1) {
            let core_ids = core_ids.clone();
            let done = done.clone();
            let original_worker_data = original_worker_data.clone();
            let shared_worker_data = shared_worker_data.clone();
            let done_preparing_shared_worker_data = done_preparing_shared_worker_data.clone();

            let join_handle = std::thread::Builder::new()
                .name(format!("thread pool worker {}", i))
                .spawn(move || {
                    // worker initial setup
                    crate::affinity::set_affinity(&core_ids);
                    let (
                        sender,
                        receiver,
                        stealer,
                    ) = Channel::<Job>::new(buffer_capacity);
                    let waker = Arc::new(ThreadWaker(std::thread::current()));
                    original_worker_data.lock()[i] = Some((stealer, waker));

                    while !done_preparing_shared_worker_data.load(Ordering::Relaxed) {
                        std::thread::yield_now();
                    }

                    // prepare worker
                    let mut g = shared_worker_data.lock();
                    let shared = ris_error::unwrap!(
                        (&mut g[i]).take().into_ris_error(),
                        "something has gone terribly wrong. this option should never be none"
                    );
                    drop(g);

                    set_worker(Worker{
                        done,
                        sender,
                        receiver,
                        shared,
                    });

                    let worker = ris_error::unwrap!(
                        get_worker().into_ris_error(),
                        "something has gone terribly wrong. this option should never be none"
                    );

                    // run worker
                    worker.run();
                })?;
            join_handles.push(join_handle);
        }

        // wait until all workers have set up their channels
        for i in 0..threads {
            while original_worker_data.lock()[i].is_none() {
                std::thread::yield_now();
            }
        }

        // all channels are setup, stealers can now be copied
        for i in 0..threads {
            let mut shared = Vec::new();
            let g = original_worker_data.lock();
            for j in 0..g.len() {
                // offset j, such that every worker has a different stealer at first, this attempts
                // to reduce contention
                let j = (j + i) % threads; 
                if i == j {
                    continue;
                }

                let original = &g[j];
                let (stealer, waker) = ris_error::unwrap!(
                    original.as_ref().into_ris_error(),
                    "something has gone terribly wrong. this option should never be none"
                );

                
                shared.push(SharedWorkerData{
                    stealer: stealer.clone(),
                    waker: waker.clone(),
                });
            }
            drop(g);
            shared_worker_data.lock().push(Some(shared));
        }

        done_preparing_shared_worker_data.store(true, Ordering::Relaxed);

        // prepare main worker
        let mut g = shared_worker_data.lock();
        let shared = ris_error::unwrap!(
            (&mut g[0]).take().into_ris_error(),
            "something has gone terribly wrong. this option should never be none"
        );
        drop(g);

        let worker = Worker {
            done: done.clone(),
            sender,
            receiver,
            shared,
        };
        set_worker(worker);

        // return thread pool
        Ok(Self {
            done,
            join_handles: Some(join_handles),
        })
    }

    pub fn submit<F: Future>(&self, future: F) -> Result<(), F> {
        let Some(worker) = get_worker() else {
            println!("not a worker");
            return Err(future); // todo better error
        };

        let job = async {
            let output = future.await;
        };

        // todo: enqueue `job` into thread pool
        // todo: find a way to return `output`

        worker.wake_other_workers();

        Ok(())
    }

    pub fn run_pending_job() -> bool {
        let Some(worker) = get_worker() else {
            return false;
        };

        worker.run_pending_job()
    }
}


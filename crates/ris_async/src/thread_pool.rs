use std::cell::RefCell;
use std::cell::UnsafeCell;
use std::marker::PhantomData;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::future::Future;
use std::thread::JoinHandle;

use ris_error::Extensions;
use ris_error::RisResult;

use crate::Channel;
use crate::Sender;
use crate::Stealer;
use crate::Receiver;
use crate::SpinLock;

pub const DEFAULT_BUFFER_CAPACITY: usize = 1024;

type Job = Box<dyn Future<Output = ()>>;

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
    stealers: Vec<Arc<Stealer<Job>>>,
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
    }

    fn run_pending_job(&self) -> bool {
        match self.receiver.receive() {
            Some(job) => true,
            None => {
                for stealer in self.stealers.iter() {
                    let Some(job) = stealer.steal() else {
                        continue;
                    };

                    return true;
                }

                false
            }
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
                while let Some(job) = worker.receiver.receive() {

                }
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
                        Err(e) => println!("failed to join thread {}", i),
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
        let stealer_originals = Arc::new(SpinLock::new(Vec::new()));
        let stealer_copies = Arc::new(SpinLock::new(Vec::<Option<_>>::new()));
        let copied_stealers = Arc::new(AtomicBool::new(false));

        let mut g = stealer_originals.lock();
        for _ in 0..threads {
            g.push(None);
        }
        drop(g);

        // setup main thread
        crate::affinity::set_affinity(&affinities[0]);
        let (
            sender,
            receiver,
            stealer,
        ) = Channel::<Job>::new(buffer_capacity);
        stealer_originals.lock()[0] = Some(stealer);

        // setup worker threads
        let done = Arc::new(AtomicBool::new(false));
        
        let mut join_handles = Vec::with_capacity(threads - 1);
        for (i, core_ids) in affinities.iter().enumerate().take(threads).skip(1) {
            let core_ids = core_ids.clone();
            let done = done.clone();
            let stealer_originals = stealer_originals.clone();
            let stealer_copies = stealer_copies.clone();
            let copied_stealers = copied_stealers.clone();

            let join_handle = std::thread::Builder::new()
                .name(format!("thread pool worker {}", i))
                .spawn(move || {
                    // setup worker
                    crate::affinity::set_affinity(&core_ids);
                    let (
                        sender,
                        receiver,
                        stealer,
                    ) = Channel::<Job>::new(buffer_capacity);

                    stealer_originals.lock()[i] = Some(stealer);
                    //stealer_originals.lock()[i] = Some(i);

                    while !copied_stealers.load(Ordering::Relaxed) {
                        std::thread::yield_now();
                    }

                    let mut g = stealer_copies.lock();
                    let stealers = ris_error::unwrap!(
                        (&mut g[i]).take().into_ris_error(),
                        "something has gone terribly wrong. this option should never be none"
                    );
                    drop(g);

                    // prepare worker and run
                    set_worker(Worker{
                        done,
                        sender,
                        receiver,
                        stealers,
                    });

                    let worker = ris_error::unwrap!(
                        get_worker().into_ris_error(),
                        "something has gone terribly wrong. this option should never be none"
                    );
                    worker.run();
                })?;
            join_handles.push(join_handle);
        }

        // wait until all workers have set up their channels
        for i in 0..threads {
            while stealer_originals.lock()[i].is_none() {
                std::thread::yield_now();
            }
        }

        // all channels are setup, stealers can now be copied
        for i in 0..threads {
            let mut copy = Vec::new();
            let g = stealer_originals.lock();
            for j in 0..g.len() {
                // offset j, such that every worker has a different stealer at first, this attempts
                // to reduce contention
                let j = (j + i) % threads; 
                if i == j {
                    continue;
                }

                let original = &g[j];
                let original_ref = ris_error::unwrap!(
                    original.as_ref().into_ris_error(),
                    "something has gone terribly wrong. this option should never be none"
                );

                copy.push(original_ref.clone());
            }
            drop(g);
            stealer_copies.lock().push(Some(copy));
        }

        copied_stealers.store(true, Ordering::Relaxed);

        let mut g = stealer_copies.lock();
        let stealers = ris_error::unwrap!(
            (&mut g[0]).take().into_ris_error(),
            "something has gone terribly wrong. this option should never be none"
        );
        drop(g);

        // prepare main worker and thread pool
        let worker = Worker {
            done: done.clone(),
            sender,
            receiver,
            stealers,
        };
        set_worker(worker);

        Ok(Self {
            done,
            join_handles: Some(join_handles),
        })
    }

    pub fn submit<F: Future>(&self, future: F) {
        let job = async {
            let output = future.await;
        };

        // todo: enqueue `job` into thread pool
        // todo: find a way to return `output`
        // todo: unpark all threads
    }

    pub fn run_pending_job() -> bool {
        let Some(worker) = get_worker() else {
            return false;
        };

        worker.run_pending_job()
    }
}


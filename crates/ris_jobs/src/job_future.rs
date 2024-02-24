use std::cell::UnsafeCell;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::time::Duration;
use std::time::Instant;

use crate::job_system;

pub struct SettableJobFuture<T> {
    is_ready: Arc<AtomicBool>,
    data: Arc<UnsafeCell<Option<T>>>,
}

pub struct JobFuture<T> {
    is_ready: Arc<AtomicBool>,
    data: Arc<UnsafeCell<Option<T>>>,
}

#[derive(Clone)]
pub struct JobFence {
    is_ready: Arc<AtomicBool>,
}

#[derive(Debug)]
pub struct TimeoutError {
    waited: Duration,
}

impl std::fmt::Display for TimeoutError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "timed out after waiting for {}ms",
            self.waited.as_millis()
        )
    }
}

impl std::error::Error for TimeoutError {}

impl<T> SettableJobFuture<T> {
    pub fn new() -> (SettableJobFuture<T>, JobFuture<T>) {
        let is_ready = Arc::new(AtomicBool::new(false));
        let data = Arc::new(UnsafeCell::new(None));

        let settable_job_future = SettableJobFuture {
            is_ready: is_ready.clone(),
            data: data.clone(),
        };
        let job_future = JobFuture { is_ready, data };

        (settable_job_future, job_future)
    }

    pub fn set(self, result: T) {
        unsafe { *self.data.get() = Some(result) };
        self.is_ready.store(true, Ordering::SeqCst);
    }
}

impl<T> JobFuture<T> {
    pub fn wait(self, timeout: Option<Duration>) -> Result<T, TimeoutError> {
        match timeout {
            Some(timeout) => spinlock_with_timeout(self.is_ready.clone(), timeout)?,
            None => spinlock(self.is_ready.clone()),
        }

        let result = unsafe { (*self.data.get()).take() };
        match result {
            Some(value) => Ok(value),
            None => unreachable!(),
        }
    }

    pub fn fence(&self) -> JobFence {
        let is_ready = self.is_ready.clone();
        JobFence { is_ready }
    }
}

impl<T: Default> JobFuture<T> {
    pub fn done() -> Self {
        let is_ready = Arc::new(AtomicBool::new(true));
        let data = Arc::new(UnsafeCell::new(Some(T::default())));

        Self { is_ready, data }
    }
}

impl JobFence {
    pub fn wait(self, timeout: Option<Duration>) -> Result<(), TimeoutError> {
        match timeout {
            Some(timeout) => spinlock_with_timeout(self.is_ready.clone(), timeout),
            None => {
                spinlock(self.is_ready.clone());
                Ok(())
            }
        }
    }

    pub fn done() -> Self {
        let is_ready = Arc::new(AtomicBool::new(true));

        Self { is_ready }
    }
}

fn spinlock(is_ready: Arc<AtomicBool>) {
    while !is_ready.load(Ordering::SeqCst) {
        job_system::run_pending_job(file!(), line!());
    }
}

fn spinlock_with_timeout(is_ready: Arc<AtomicBool>, timout: Duration) -> Result<(), TimeoutError> {
    let start = Instant::now();

    while !is_ready.load(Ordering::SeqCst) {
        job_system::run_pending_job(file!(), line!());

        let now = Instant::now();
        let duration = now - start;

        if duration > timout {
            return Err(TimeoutError { waited: duration });
        }
    }

    Ok(())
}

unsafe impl<T> Send for SettableJobFuture<T> {}
unsafe impl<T> Sync for SettableJobFuture<T> {}
unsafe impl<T> Send for JobFuture<T> {}
unsafe impl<T> Sync for JobFuture<T> {}
unsafe impl Send for JobFence {}
unsafe impl Sync for JobFence {}

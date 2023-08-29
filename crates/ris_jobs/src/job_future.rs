use std::sync::Arc;
use std::sync::Mutex;
use std::sync::TryLockError;

use ris_util::throw;

use crate::job_system;

struct Inner<T> {
    is_ready: bool,
    data: Option<T>,
}

type InnerPtr<T> = Arc<Mutex<Inner<T>>>;

pub struct SettableJobFuture<T> {
    inner: InnerPtr<T>,
}

pub struct JobFuture<T> {
    inner: InnerPtr<T>,
}

impl<T> SettableJobFuture<T> {
    pub fn new() -> (SettableJobFuture<T>, JobFuture<T>) {
        let inner = Arc::new(Mutex::new(Inner {
            is_ready: false,
            data: None,
        }));

        let settable_job_future = SettableJobFuture {
            inner: inner.clone(),
        };
        let job_future = JobFuture { inner };

        (settable_job_future, job_future)
    }

    pub fn set(self, result: T) {
        let mut inner = job_system::lock(&self.inner);

        inner.is_ready = true;
        inner.data = Some(result);
    }
}

impl<T> JobFuture<T> {
    pub fn wait(mut self) -> T {
        match self.wait_and_take() {
            Some(value) => value,
            None => unreachable!(),
        }
    }

    fn wait_and_take(&mut self) -> Option<T> {
        loop {
            match self.inner.try_lock() {
                Ok(mut inner) => {
                    if inner.is_ready {
                        return inner.data.take();
                    }
                }
                Err(e) => {
                    if let TryLockError::Poisoned(e) = e {
                        throw!("couldn't take job future: {}", e);
                    }
                }
            }

            job_system::run_pending_job();
        }
    }
}

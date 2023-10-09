use std::sync::Arc;
use std::sync::Mutex;
use std::sync::TryLockError;

use ris_util::ris_error::RisResult;

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
    pub fn new() -> RisResult<(SettableJobFuture<T>, JobFuture<T>)> {
        let inner = Arc::new(Mutex::new(Inner {
            is_ready: false,
            data: None,
        }));

        let settable_job_future = SettableJobFuture {
            inner: inner.clone(),
        };
        let job_future = JobFuture { inner };

        Ok((settable_job_future, job_future))
    }

    pub fn set(self, result: T) -> RisResult<()> {
        let mut inner = job_system::lock(&self.inner)?;

        inner.is_ready = true;
        inner.data = Some(result);

        Ok(())
    }
}

impl<T> JobFuture<T> {
    pub fn wait(mut self) -> RisResult<T> {
        match self.wait_and_take()? {
            Some(value) => Ok(value),
            None => unreachable!(),
        }
    }

    fn wait_and_take(&mut self) -> RisResult<Option<T>> {
        loop {
            match self.inner.try_lock() {
                Ok(mut inner) => {
                    if inner.is_ready {
                        return Ok(inner.data.take());
                    }
                }
                Err(e) => match e {
                    TryLockError::WouldBlock => (),
                    TryLockError::Poisoned(p) => {
                        return ris_util::result_err!(
                            "failed to take job future, because mutex was poisoned: {}",
                            p
                        );
                    }
                },
            }

            job_system::run_pending_job()?;
        }
    }
}

impl<T: Default> JobFuture<T> {
    pub fn done() -> Self {
        let inner = Arc::new(Mutex::new(Inner {
            is_ready: true,
            data: Some(T::default()),
        }));

        Self { inner }
    }
}

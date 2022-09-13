use std::sync::{Arc, Mutex, TryLockError};

use crate::{job_poll::JobPoll, job_system};

type Data<T> = Arc<Mutex<JobPoll<T>>>;

pub struct SettableJobFuture<T> {
    data: Data<T>,
}

#[must_use]
pub struct JobFuture<T> {
    data: Data<T>,
}

impl<T> SettableJobFuture<T> {
    pub fn new() -> (SettableJobFuture<T>, JobFuture<T>) {
        let data = Arc::new(Mutex::new(JobPoll::Pending));

        let settable_job_future = SettableJobFuture { data: data.clone() };
        let job_future = JobFuture { data };

        (settable_job_future, job_future)
    }

    pub fn set(&mut self, result: T) {
        self.data.lock().map_or_else(
            |e| ris_log::error!("could not lock future: {}", e),
            |mut data| *data = JobPoll::Ready(Some(result)),
        );
    }
}

impl<T> JobFuture<T> {
    pub fn wait(self) -> T {
        match self.wait_and_take() {
            Some(value) => value,
            None => unreachable!(),
        }
    }

    fn wait_and_take(&self) -> Option<T> {
        loop {
            match self.data.try_lock() {
                Ok(mut data) => {
                    if data.is_ready() {
                        return data.take();
                    }
                }
                Err(e) => {
                    if let TryLockError::Poisoned(e) = e {
                        let message = format!("could not lock future: {}", e);
                        ris_log::error!("{}", message);
                        panic!("{}", message);
                    }
                }
            }

            job_system::run_pending_job();
        }
    }
}

impl<T> Drop for JobFuture<T> {
    fn drop(&mut self) {
        self.wait_and_take();
    }
}

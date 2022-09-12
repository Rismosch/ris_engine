use std::sync::{Arc, Mutex, TryLockError};

use crate::job_poll::JobPoll;

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
            |mut data| *data = JobPoll::Ready(result),
        );
    }
}

impl<T> JobFuture<T> {
    pub fn poll(&self) -> JobPoll<T> {
        match self.data.try_lock() {
            Ok(mut data) => data.take(),
            Err(e) => {
                if let TryLockError::Poisoned(e) = e {
                    ris_log::error!("could not lock future: {}", e);
                }

                JobPoll::Pending
            }
        }
    }
}

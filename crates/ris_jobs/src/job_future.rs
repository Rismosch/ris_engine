use std::{
    ptr::NonNull,
    sync::{atomic::{AtomicBool, Ordering}, Arc, Mutex},
    task::Poll, cell::RefCell,
};

type Data<T> = Arc<Mutex<Poll<T>>>;

pub struct SettableJobFuture<T> {
    data: Data<T>,
}

pub struct JobFuture<T> {
    data: Data<T>,
}

impl<T> SettableJobFuture<T> {
    pub fn new() -> (SettableJobFuture<T>, JobFuture<T>) {
        let data = Arc::new(Mutex::new(Poll::Pending));

        let settable_job_future = SettableJobFuture { data: data.clone() };
        let job_future = JobFuture { data };

        (settable_job_future, job_future)
    }

    pub fn set(&mut self, result: T) {
        self.data.lock().map_or_else(
            |e| ris_log::error!("could not lock future: {}", e),
            |mut data| *data = Poll::Ready(result)
        );
    }
}

impl<T: Clone> JobFuture<T> {
    pub fn poll(&self) -> Poll<T> {
        self.data.lock().map_or_else(
            |e| {
                ris_log::error!("could not lock future: {}", e);
                Poll::Pending
            },
            |data| data.clone()
        )
    }
}

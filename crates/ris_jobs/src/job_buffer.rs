use std::{
    cell::UnsafeCell,
    sync::{Arc, Mutex, TryLockError},
};

use crate::{
    errors::{BlockedOrEmpty, BlockedOrFull, IsEmpty},
    job::Job,
};

pub struct JobBuffer {
    head: UnsafeCell<usize>,
    tail: Mutex<usize>,
    jobs: Vec<Mutex<Option<Job>>>,
}

impl JobBuffer {
    pub fn new(capacity: usize) -> Arc<Self> {
        let mut jobs = Vec::with_capacity(capacity);
        for _ in 0..capacity {
            jobs.push(Mutex::new(None));
        }

        Arc::new(Self {
            head: UnsafeCell::new(0),
            tail: Mutex::new(0),
            jobs,
        })
    }

    pub fn push(&self, job: Job) -> Result<(), BlockedOrFull> {
        let head = unsafe { &mut *self.head.get() };

        let mut node = match self.jobs[*head].try_lock() {
            Ok(node) => node,
            Err(std::sync::TryLockError::WouldBlock) => {
                return Err(BlockedOrFull { not_pushed: job })
            }
            Err(std::sync::TryLockError::Poisoned(_)) => panic_poisoned(),
        };

        match *node {
            Some(_) => Err(BlockedOrFull { not_pushed: job }),
            None => {
                *node = Some(job);
                *head = (*head + 1) % self.jobs.capacity();

                Ok(())
            }
        }
    }

    pub fn wait_and_pop(&self) -> Result<Job, IsEmpty> {
        let head = unsafe { &mut *self.head.get() };

        let new_head = if *head == 0 {
            self.jobs.capacity() - 1
        } else {
            *head - 1
        };

        let mut node = self.jobs[new_head].lock().map_err(|_| panic_poisoned())?;

        match node.take() {
            None => Err(IsEmpty),
            Some(job) => {
                *head = new_head;

                Ok(job)
            }
        }
    }

    pub fn steal(&self) -> Result<Job, BlockedOrEmpty> {
        let mut tail = self.tail.try_lock().map_err(to_steal_error)?;
        let old_tail = *tail;

        let mut node = self.jobs[old_tail].try_lock().map_err(to_steal_error)?;

        match node.take() {
            None => Err(BlockedOrEmpty),
            Some(job) => {
                *tail = (old_tail + 1) % self.jobs.capacity();

                Ok(job)
            }
        }
    }
}

unsafe impl Send for JobBuffer {}
unsafe impl Sync for JobBuffer {}

fn panic_poisoned() -> ! {
    let poisoned_error_message = "mutex was poisoned";
    ris_log::error!("{}", poisoned_error_message);
    panic!("{}", poisoned_error_message);
}

fn to_steal_error<T>(error: TryLockError<T>) -> BlockedOrEmpty {
    match error {
        std::sync::TryLockError::WouldBlock => BlockedOrEmpty,
        std::sync::TryLockError::Poisoned(_) => panic_poisoned(),
    }
}

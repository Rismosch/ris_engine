use std::cell::UnsafeCell;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::TryLockError;

use crate::throw;
use crate::unwrap_or_throw;

use crate::errors::BlockedOrEmpty;
use crate::errors::BlockedOrFull;
use crate::errors::IsEmpty;
use crate::job::Job;

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

    /// # Safety
    ///
    /// This function is not threadsafe. If this function and `wait_and_pop()` are called by two
    /// different threads, a race condition can concur.
    /// * Clientcode **MUST** ensure, that these two functions are never called at the same time.
    /// * It's highly recommended that clientcode calls these methods on the same thread.
    pub unsafe fn push(&self, job: Job) -> Result<(), BlockedOrFull> {
        let head = unsafe { &mut *self.head.get() };

        let mut node = match self.jobs[*head].try_lock() {
            Ok(node) => node,
            Err(std::sync::TryLockError::WouldBlock) => {
                return Err(BlockedOrFull { not_pushed: job })
            }
            Err(std::sync::TryLockError::Poisoned(e)) => throw!("mutex is poisoned: {}", e),
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

    /// # Safety
    ///
    /// This function is not threadsafe. If this function and `push()` are called by two different
    /// threads a race condition can concur.
    /// * Clientcode **MUST** ensure, that these two functions are never called at the same time.
    /// * It's highly recommended that clientcode calls these methods on the same thread.
    pub unsafe fn wait_and_pop(&self) -> Result<Job, IsEmpty> {
        let head = unsafe { &mut *self.head.get() };

        let new_head = if *head == 0 {
            self.jobs.capacity() - 1
        } else {
            *head - 1
        };

        let mut node = unwrap_or_throw!(self.jobs[new_head].lock(), "mutex is poisoned");

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

fn to_steal_error<T>(error: TryLockError<T>) -> BlockedOrEmpty {
    match error {
        std::sync::TryLockError::WouldBlock => BlockedOrEmpty,
        std::sync::TryLockError::Poisoned(e) => throw!("mutex is poisoned: {}", e),
    }
}

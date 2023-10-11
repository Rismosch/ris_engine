use std::cell::UnsafeCell;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::MutexGuard;
use std::sync::PoisonError;
use std::sync::TryLockError;

use crate::job::Job;

pub type JobPoisonError<'a> = PoisonError<MutexGuard<'a, Option<Job>>>;
pub type TailPoisonError<'a> = PoisonError<MutexGuard<'a, usize>>;

#[derive(Debug)]
pub enum PushError<'a> {
    BlockedOrFull(Job),
    MutexPoisoned(JobPoisonError<'a>),
}

#[derive(Debug)]
pub enum PopError<'a> {
    IsEmpty,
    MutexPoisoned(JobPoisonError<'a>),
}

#[derive(Debug)]
pub enum StealError<'a> {
    BlockedOrEmpty,
    TailPoisoned(TailPoisonError<'a>),
    JobPoisoned(JobPoisonError<'a>),
}

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
    pub unsafe fn push(&self, job: Job) -> Result<(), PushError> {
        let head = unsafe { &mut *self.head.get() };

        let mut node = match self.jobs[*head].try_lock() {
            Ok(node) => node,
            Err(TryLockError::WouldBlock) => return Err(PushError::BlockedOrFull(job)),
            Err(TryLockError::Poisoned(e)) => return Err(PushError::MutexPoisoned(e)),
        };

        match *node {
            Some(_) => Err(PushError::BlockedOrFull(job)),
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
    pub unsafe fn wait_and_pop(&self) -> Result<Job, PopError> {
        let head = unsafe { &mut *self.head.get() };

        let new_head = if *head == 0 {
            self.jobs.capacity() - 1
        } else {
            *head - 1
        };

        let mut node = self.jobs[new_head]
            .lock()
            .map_err(PopError::MutexPoisoned)?;

        match node.take() {
            None => Err(PopError::IsEmpty),
            Some(job) => {
                *head = new_head;

                Ok(job)
            }
        }
    }

    pub fn steal(&self) -> Result<Job, StealError> {
        let mut tail = self.tail.try_lock().map_err(|e| match e {
            TryLockError::WouldBlock => StealError::BlockedOrEmpty,
            TryLockError::Poisoned(p) => StealError::TailPoisoned(p),
        })?;
        let old_tail = *tail;

        let mut node = self.jobs[old_tail].try_lock().map_err(|e| match e {
            TryLockError::WouldBlock => StealError::BlockedOrEmpty,
            TryLockError::Poisoned(p) => StealError::JobPoisoned(p),
        })?;

        match node.take() {
            None => Err(StealError::BlockedOrEmpty),
            Some(job) => {
                *tail = (old_tail + 1) % self.jobs.capacity();

                Ok(job)
            }
        }
    }
}

unsafe impl Send for JobBuffer {}
unsafe impl Sync for JobBuffer {}

use std::{
    ops::{Deref, DerefMut},
    ptr::NonNull,
    sync::{
        atomic::{AtomicI32, Ordering},
        Mutex, TryLockError,
    },
};

use crate::{
    errors::{BlockedOrEmpty, BlockedOrFull, IsEmpty},
    job::Job,
};

pub struct JobBuffer {
    inner: NonNull<Inner>,
}

struct Inner {
    head: usize,
    tail: Mutex<usize>,
    jobs: Vec<Mutex<Option<Job>>>,
    refs: AtomicI32,
}

impl JobBuffer {
    pub fn new(capacity: usize) -> Self {
        let mut jobs = Vec::with_capacity(capacity);
        for _ in 0..capacity {
            jobs.push(Mutex::new(None));
        }

        let boxed = Box::into_raw(Box::new(Inner {
            head: 0,
            tail: Mutex::new(0),
            jobs,
            refs: AtomicI32::new(0),
        }));

        let inner = unsafe { NonNull::new_unchecked(boxed) };

        Self { inner }
    }

    pub fn duplicate(&mut self) -> Self {
        let inner = self.inner();

        let _previous_refs = inner.refs.fetch_add(1, Ordering::SeqCst);

        Self { inner: self.inner }
    }

    pub fn push(&mut self, job: Job) -> Result<(), BlockedOrFull> {
        let inner = self.inner();

        let mut node = match inner.jobs[inner.head].try_lock() {
            Ok(node) => node,
            Err(std::sync::TryLockError::WouldBlock) => {
                return Err(BlockedOrFull { not_pushed: job })
            }
            Err(std::sync::TryLockError::Poisoned(_)) => panic_poisoned(),
        };

        match *node.deref() {
            Some(_) => Err(BlockedOrFull { not_pushed: job }),
            None => {
                *node.deref_mut() = Some(job);
                inner.head = (inner.head + 1) % inner.jobs.capacity();

                Ok(())
            }
        }
    }

    pub fn wait_and_pop(&mut self) -> Result<Job, IsEmpty> {
        let inner = self.inner();

        let new_head = if inner.head == 0 {
            inner.jobs.capacity() - 1
        } else {
            inner.head - 1
        };

        let mut node = inner.jobs[new_head].lock().map_err(|_| panic_poisoned())?;

        match node.deref_mut().take() {
            None => Err(IsEmpty),
            Some(job) => {
                inner.head = new_head;

                Ok(job)
            }
        }
    }

    pub fn steal(&mut self) -> Result<Job, BlockedOrEmpty> {
        let inner = self.inner();

        let mut tail = inner.tail.try_lock().map_err(to_steal_error)?;
        let old_tail = *tail;

        let mut node = inner.jobs[old_tail].try_lock().map_err(to_steal_error)?;

        match node.deref_mut().take() {
            None => Err(BlockedOrEmpty),
            Some(job) => {
                *tail = (old_tail + 1) % inner.jobs.capacity();

                Ok(job)
            }
        }
    }

    fn inner(&mut self) -> &mut Inner {
        unsafe { &mut *self.inner.as_ptr() }
    }
}

impl Drop for JobBuffer {
    fn drop(&mut self) {
        let inner = self.inner();

        let previous_refs = inner.refs.fetch_sub(1, Ordering::SeqCst);
        if previous_refs < 1 {
            unsafe {
                Box::from_raw(inner);
            }
        }
    }
}

unsafe impl Send for JobBuffer {}
// unsafe impl Sync for JobBuffer {}

fn panic_poisoned() -> ! {
    let poisoned_error_message = "mutex was poisoned";
    ris_log::fatal!("{}", poisoned_error_message);
    panic!("{}", poisoned_error_message);
}

fn to_steal_error<T>(error: TryLockError<T>) -> BlockedOrEmpty {
    match error {
        std::sync::TryLockError::WouldBlock => BlockedOrEmpty,
        std::sync::TryLockError::Poisoned(_) => panic_poisoned(),
    }
}

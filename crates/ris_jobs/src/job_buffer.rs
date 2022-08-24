use std::{
    ops::{Deref, DerefMut},
    ptr::NonNull,
    sync::{
        atomic::{AtomicI32, Ordering},
        Mutex,
    },
};

use crate::{
    errors::{IsEmpty, IsFull},
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

    pub fn push(&mut self, job: Job) -> Result<(), IsFull> {
        let inner = self.inner();

        let mut node = inner.jobs[inner.head].lock().unwrap();

        match *node.deref() {
            Some(_) => Err(IsFull {
                not_pushed_job: job,
            }),
            None => {
                *node.deref_mut() = Some(job);
                inner.head = (inner.head + 1) % inner.jobs.capacity();

                Ok(())
            }
        }
    }

    pub fn pop(&mut self) -> Result<Job, IsEmpty> {
        let inner = self.inner();

        let new_head = if inner.head == 0 {
            inner.jobs.capacity() - 1
        } else {
            inner.head - 1
        };

        let mut node = inner.jobs[new_head].lock().unwrap();

        match node.deref_mut().take() {
            None => Err(IsEmpty),
            Some(job) => {
                inner.head = new_head;

                Ok(job)
            }
        }
    }

    pub fn steal(&mut self) -> Result<Job, IsEmpty> {
        let inner = self.inner();

        let mut tail = inner.tail.lock().unwrap();
        let old_tail = *tail;

        let mut node = inner.jobs[old_tail].lock().unwrap();

        match node.deref_mut().take() {
            None => Err(IsEmpty),
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

use std::{
    ops::{Deref, DerefMut},
    sync::{Mutex, atomic::{AtomicI32, Ordering}}, ptr::NonNull,
};

use crate::{job::Job, errors::{IsFull, IsEmpty}};

pub struct JobBuffer {
    inner: NonNull<Inner>,
}

struct Inner {
    head: Mutex<usize>,
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
            head: Mutex::new(0),
            tail: Mutex::new(0),
            jobs,
            refs: AtomicI32::new(0),
        }));

        let inner = unsafe {
            NonNull::new_unchecked(boxed)
        };

        Self { inner }
    }

    pub fn duplicate(&self) -> Self {
        let inner = self.inner();

        inner.refs.fetch_add(1, Ordering::SeqCst);

        Self { inner: self.inner }
    }

    pub fn push(&mut self, job: Job) -> Result<(), IsFull> {
        let inner = self.inner();

        let mut head = inner.head.lock().unwrap();
        let old_head = *head;
        let mut node = inner.jobs[old_head].lock().unwrap();

        match *node.deref() {
            Some(_) => Err(IsFull{not_pushed_job: job}),
            None => {
                *node.deref_mut() = Some(job);
                *head = (old_head + 1) % inner.jobs.capacity();

                Ok(())
            }
        }
    }

    pub fn pop(&mut self) -> Result<Job, IsEmpty> {
        let inner = self.inner();

        let mut head = inner.head.lock().unwrap();
        let old_head = *head;
        let new_head = if old_head == 0 {
            inner.jobs.capacity() - 1
        } else {
            old_head - 1
        };

        let mut node = inner.jobs[new_head].lock().unwrap();

        match node.deref_mut().take() {
            None => Err(IsEmpty),
            Some(job) => {
                *head = new_head;

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

    fn inner(&self) -> &mut Inner {
        unsafe {
            &mut *self.inner.as_ptr()
        }
    }
}

impl Drop for JobBuffer {
    fn drop(&mut self) {
        unsafe {
            let inner = &mut *self.inner.as_ptr();

            let previous_refs = inner.refs.fetch_sub(1, Ordering::SeqCst);
            if previous_refs < 1 {
                let _ = Box::from_raw(inner);
            }
        }
    }
}

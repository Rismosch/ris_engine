use std::{task::Poll, sync::atomic::{Ordering, AtomicBool}, ptr::NonNull};

pub struct  SettableJobFuture<T: Clone> {
    inner: NonNull<Inner<T>>,
}

pub struct JobFuture<T: Clone> {
    inner: NonNull<Inner<T>>,
}

pub struct Inner<T> {
    poll: Poll<T>,
    can_be_dropped: AtomicBool,
}

impl<T: Clone> SettableJobFuture<T> {
    pub fn new() -> (SettableJobFuture<T>, JobFuture<T>) {
        let boxed = Box::into_raw(Box::new(Inner{
            poll: Poll::Pending,
            can_be_dropped: AtomicBool::new(false),
        }));

        let inner = unsafe { NonNull::new_unchecked(boxed) };

        let settable_job_future = SettableJobFuture { inner };
        let job_future = JobFuture { inner };

        (settable_job_future, job_future)
    }

    pub fn set(&mut self, result: T) {
        let inner = unsafe { &mut *self.inner.as_ptr() };
        inner.poll = Poll::Ready(result);
    }
}

impl<T: Clone> JobFuture<T> {
    pub fn poll(&self) -> &Poll<T> {
        let inner = unsafe { & *self.inner.as_ptr() };

        &inner.poll
    }
}

impl<T: Clone> Drop for SettableJobFuture<T> {
    fn drop(&mut self) {
        let inner = unsafe { &mut *self.inner.as_ptr() };

        drop_inner(inner);
    }
}

impl<T: Clone> Drop for JobFuture<T> {
    fn drop(&mut self) {
        let inner = unsafe { &mut *self.inner.as_ptr() };
        
        drop_inner(inner);
    }
}

fn drop_inner<T>(inner: &mut Inner<T>) {
    let can_be_dropped = inner.can_be_dropped.fetch_or(true, Ordering::SeqCst);
    if can_be_dropped {
        unsafe {
            Box::from_raw(inner);
        }
    }
}

unsafe impl<T: Clone> Send for SettableJobFuture<T> {}
unsafe impl<T: Clone> Send for JobFuture<T> {}
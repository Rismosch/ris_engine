use std::{sync::atomic::{AtomicBool, AtomicI32, Ordering}, ptr::NonNull};

use crate::job_system;

struct Inner<T> {
    is_ready: AtomicBool,
    data: Option<T>,
    refs: AtomicI32,
}

type InnerPtr<T> = NonNull<Inner<T>>;

pub struct SettableJobFuture<T> {
    inner: InnerPtr<T>,
}

#[must_use]
pub struct JobFuture<T> {
    inner: InnerPtr<T>,
}

impl<T> SettableJobFuture<T> {
    pub fn new() -> (SettableJobFuture<T>, JobFuture<T>) {

        let boxed = Box::into_raw(Box::new(Inner{
            is_ready: AtomicBool::new(false),
            data: None,
            refs: AtomicI32::new(1),
        }));

        let inner_a = unsafe {NonNull::new_unchecked(boxed)};
        let inner_b = unsafe {NonNull::new_unchecked(boxed)};

        let settable_job_future = SettableJobFuture { inner: inner_a };
        let job_future = JobFuture { inner: inner_b };

        (settable_job_future, job_future)
    }

    pub fn set(mut self, result: T) {
        let inner = deref_inner(&mut self.inner);
        inner.data = Some(result);
        inner.is_ready.store(true, Ordering::SeqCst);
    }
}

impl<T> JobFuture<T> {
    pub fn wait(mut self) -> T {
        match self.wait_and_take() {
            Some(value) => value,
            None => unreachable!(),
        }
    }

    fn wait_and_take(&mut self) -> Option<T> {
        let inner = deref_inner(&mut self.inner);

        while !inner.is_ready.load(Ordering::SeqCst) {
            job_system::run_pending_job();
        }

        inner.data.take()
    }
}

impl<T> Drop for SettableJobFuture<T> {
    fn drop(&mut self) {
        drop_inner(&mut self.inner);
    }
}

impl<T> Drop for JobFuture<T> {
    fn drop(&mut self) {
        self.wait_and_take();

        drop_inner(&mut self.inner);
    }
}

unsafe impl<T> Send for SettableJobFuture<T> {}
unsafe impl<T> Send for JobFuture<T> {}

fn deref_inner<T>(data: &mut InnerPtr<T>) -> &mut Inner<T> {
    unsafe {&mut *data.as_ptr()}
}

fn drop_inner<T>(inner: &mut InnerPtr<T>) {
    let inner = deref_inner(inner);

    let previous_refs = inner.refs.fetch_sub(1, Ordering::SeqCst);
    if previous_refs < 1 {
        unsafe {
            Box::from_raw(inner);
        }
    }
}

use std::cell::UnsafeCell;
use std::mem::MaybeUninit;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::future::Future;
use std::pin::Pin;
use std::task::Context;
use std::task::Poll;

struct JobFutureInner<T> {
    ready: AtomicBool,
    data: UnsafeCell<MaybeUninit<T>>,
}

pub struct JobFuture<T> {
    inner: Arc<JobFutureInner<T>>,
}

pub struct JobFutureSetter<T> {
    inner: Arc<JobFutureInner<T>>,
}

impl<T> Future for JobFuture<T> {
    type Output = T;

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.as_mut().inner.ready.swap(false, Ordering::Acquire) {
            let output = unsafe{(*self.inner.data.get()).assume_init_read()};
            Poll::Ready(output)
        } else {
            Poll::Pending
        }
    }
}

impl<T> JobFuture<T> {
    pub fn new() -> (Self, JobFutureSetter<T>) {
        let inner = Arc::new(JobFutureInner{
            ready: AtomicBool::new(false),
            data: UnsafeCell::new(MaybeUninit::uninit()),
        });

        let future = Self {inner: inner.clone()};
        let setter = JobFutureSetter{inner};
        (future, setter)
    }
}

impl<T> JobFutureSetter<T> {
    pub fn set(self, value: T) {
        unsafe {(*self.inner.data.get()).write(value)};
        self.inner.ready.store(true, Ordering::Release);
    }
}

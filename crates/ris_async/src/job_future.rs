use std::cell::UnsafeCell;
use std::future::Future;
use std::mem::MaybeUninit;
use std::pin::Pin;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::task::Context;
use std::task::Poll;
use std::task::Wake;

use crate::ThreadPool;

struct EmptyWaker;

impl Wake for EmptyWaker {
    fn wake(self: Arc<Self>) {}
}

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

unsafe impl<T> Sync for JobFutureInner<T> where T: Send {}

impl<T> Future for JobFuture<T> {
    type Output = T;

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.as_mut().inner.ready.swap(false, Ordering::Acquire) {
            let output = unsafe { (*self.inner.data.get()).assume_init_read() };
            Poll::Ready(output)
        } else {
            Poll::Pending
        }
    }
}

impl<T> JobFuture<T> {
    pub fn new() -> (Self, JobFutureSetter<T>) {
        let inner = Arc::new(JobFutureInner {
            ready: AtomicBool::new(false),
            data: UnsafeCell::new(MaybeUninit::uninit()),
        });

        let future = Self {
            inner: inner.clone(),
        };
        let setter = JobFutureSetter { inner };
        (future, setter)
    }

    pub fn wait(self) -> T {
        ThreadPool::block_on(self)
    }
}

impl<T> JobFutureSetter<T> {
    pub fn set(self, value: T) {
        unsafe { (*self.inner.data.get()).write(value) };
        self.inner.ready.store(true, Ordering::Release);
    }
}

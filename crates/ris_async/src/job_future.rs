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
use crate::oneshot_channel;
use crate::OneshotSender;
use crate::OneshotReceiver;

struct EmptyWaker;

impl Wake for EmptyWaker {
    fn wake(self: Arc<Self>) {}
}

pub struct JobFuture<T> {
    receiver: OneshotReceiver<T>,
}

pub struct JobFutureSetter<T> {
    sender: OneshotSender<T>,
}

impl<T> Future for JobFuture<T> {
    type Output = T;

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.as_mut().receiver.take() {
            Some(output) => Poll::Ready(output),
            None => Poll::Pending,
        }
    }
}

impl<T> JobFuture<T> {
    pub fn new() -> (Self, JobFutureSetter<T>) {
        let (sender, receiver) = oneshot_channel();
        let future = Self{receiver};
        let setter = JobFutureSetter{sender};
        (future, setter)
    }

    pub fn wait(self) -> T {
        ThreadPool::block_on(self)
    }
}

impl<T> JobFutureSetter<T> {
    pub fn set(self, value: T) {
        self.sender.send(value);
    }
}

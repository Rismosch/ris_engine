use std::future::Future;
use std::pin::Pin;
use std::task::Context;
use std::task::Poll;

use crate::oneshot_channel;
use crate::OneshotReceiver;
use crate::OneshotSender;
use crate::ThreadPool;

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
        let future = Self { receiver };
        let setter = JobFutureSetter { sender };
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

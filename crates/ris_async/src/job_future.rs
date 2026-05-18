use std::future::Future;
use std::pin::Pin;
use std::task::Context;
use std::task::Poll;

use crate::SingleUseReceiver;
use crate::SingleUseSender;
use crate::ThreadPool;
use crate::single_use_channel;

#[must_use]
pub struct JobFuture<T> {
    receiver: SingleUseReceiver<T>,
}

pub struct JobFutureSetter<T> {
    sender: SingleUseSender<T>,
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
    pub fn finished(value: T) -> Self {
        let receiver = SingleUseReceiver::with_value(value);
        Self { receiver }
    }

    pub fn new() -> (Self, JobFutureSetter<T>) {
        let (sender, receiver) = single_use_channel();
        let future = Self { receiver };
        let setter = JobFutureSetter { sender };
        (future, setter)
    }

    pub fn wait(self) -> T {
        ThreadPool::block_on(self)
    }

    pub fn ignore(self) {}
}

impl<T> JobFutureSetter<T> {
    pub fn set(self, value: T) {
        self.sender.send(value);
    }
}

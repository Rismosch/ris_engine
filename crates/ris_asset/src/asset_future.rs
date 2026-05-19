use std::future::Future;
use std::marker::PhantomData;
use std::pin::Pin;
use std::task::Context;
use std::task::Poll;

use ris_async::UnsafeReceiver;
use ris_async::UnsafeSender;
use ris_async::ThreadPool;

use crate::assets::ris_asset::RisAsset;

/// must be awaited, or the underlying data will be leaked
#[must_use]
pub struct AssetFuture<T: RisAsset> {
    receiver: UnsafeReceiver,
    _boo: PhantomData<T>,
}

pub struct AssetFutureSetter {
    sender: UnsafeSender,
}

impl<T: RisAsset> Unpin for AssetFuture<T> {}

impl<T: RisAsset> Future for AssetFuture<T> {
    type Output = Box<T>;

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        match unsafe{self.as_mut().receiver.take()} {
            Some(output) => Poll::Ready(output),
            None => Poll::Pending,
        }
    }
}

impl<T: RisAsset> AssetFuture<T> {
    pub fn new() -> (Self, AssetFutureSetter) {
        let (sender, receiver) = ris_async::unsafe_channel::<T>();
        let future = Self { receiver, _boo: PhantomData };
        let setter = AssetFutureSetter { sender };
        (future, setter)
    }

    pub fn wait(self) -> Box<T> {
        ThreadPool::block_on(self)
    }
}

impl AssetFutureSetter {
    pub fn sender(&self) -> &UnsafeSender {
        &self.sender
    }
}

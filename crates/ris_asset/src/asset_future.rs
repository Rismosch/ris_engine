use std::cell::UnsafeCell;
use std::ffi::c_void;
use std::future::Future;
use std::mem::MaybeUninit;
use std::pin::Pin;
use std::ptr::NonNull;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::task::Context;
use std::task::Poll;

use ris_async::UnsafeReceiver;
use ris_async::UnsafeSender;
use ris_async::ThreadPool;

use crate::assets::ris_asset::RisAsset;

#[must_use]
pub struct AssetFuture<T: RisAsset> {
    receiver: UnsafeReceiver,
    data: UnsafeCell<MaybeUninit<T>>,
}

pub struct AssetFutureSetter {
    sender: UnsafeSender,
}

impl<T: RisAsset> Drop for AssetFuture<T> {
    fn drop(&mut self) {
        ris_error::panic!("todo: safety check: setter must be dropped before dropping future. setter may not outlive future")
    }
}

impl<T: RisAsset> Future for AssetFuture<T> {
    type Output = T;

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        if unsafe {!self.as_mut().receiver.is_init()} {
            return Poll::Pending;
        }

        todo!();
    }
}

impl<T: RisAsset> AssetFuture<T> {
    pub fn new() -> (Self, AssetFutureSetter) {
        let mut data = UnsafeCell::new(MaybeUninit::uninit());
        let p_data = unsafe {NonNull::<T>::new_unchecked(data.get_mut().as_mut_ptr())};
        let (sender, receiver) = ris_async::unsafe_channel(p_data);
        let future = Self { receiver, data };
        let setter = AssetFutureSetter { sender };
        (future, setter)
    }

    pub fn wait(self) -> T {
        ThreadPool::block_on(self)
    }
}

impl AssetFutureSetter {
    pub fn sender(&self) -> &UnsafeSender {
        &self.sender
    }
}

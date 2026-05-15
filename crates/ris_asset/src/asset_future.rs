use std::ffi::c_void;
use std::future;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

use ris_async::ThreadPool;

use crate::assets::ris_asset::RisAsset;

#[must_use]
pub struct AssetFuture<T: RisAsset> {
    ready: Arc<AtomicBool>,
    data: T,
}

pub struct AssetFutureSetter {
    ready: Arc<AtomicBool>,
    p_data: *mut c_void,
}

impl<T: RisAsset> Future for AssetFuture<T> {
    type Output = T;

    fn poll(mut self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        match self.as_mut().take() {
            Some(output) => std::task::Poll::Ready(output),
            None => std::task::Poll::Pending,
        }
}

impl<T: RisAsset> AssetFuture<T> {
    pub fn new() -> (Self, AssetFutureSetter) {
        let future = AssetFuture {
            ready: Arc::new(AtomicBool::new(false)),
            data: std::mem::zeroed(),
        };

        let setter = AssetFutureSetter {
            ready: future.ready.clone(),
            p_data: &mut future.data as *mut c_void,
        };

        (future, setter)
    }

    pub fn receive(mut self) -> Result<T, Self> {
        match self.take() {
            Some(value) => Ok(value),
            None => Err(self),
        }
    }

    fn take(&mut self) -> Option<T> {
        if self.ready.swap(false, Ordering::Acquire) {
            Some(self.data)
        } else {
            None
        }
    }

    pub fn wait(self) -> T {
        ThreadPool::block_on(self)
    }
}

impl AssetFutureSetter {
    /// # Safety
    ///
    /// `AssetFuture` MUST store a `T`. Violating this causes undefined behaviour.
    unsafe fn set<T: RisAsset>(self, value: T) {
        let p_data = self.p_data as *mut T;
        unsafe { *p_data = value};
        self.ready.store(true, Ordering::Release);
    }
}

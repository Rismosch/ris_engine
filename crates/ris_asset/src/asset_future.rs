use std::marker::PhantomData;

use ris_async::ThreadPool;
use ris_async::UnsafeReceiver;
use ris_async::UnsafeSender;

use crate::assets::ris_asset::RisAsset;

/// must be awaited, or the underlying data will be leaked
#[must_use]
pub struct AssetFuture<T: RisAsset> {
    receiver: UnsafeReceiver,
    _boo: PhantomData<T>,
}

impl<T: RisAsset> AssetFuture<T> {
    pub fn new() -> (Self, UnsafeSender) {
        let (sender, receiver) = ris_async::unsafe_channel::<T>();
        let future = Self { receiver, _boo: PhantomData };
        (future, sender)
    }

    pub fn wait(mut self) -> Box<T> {
        let Some(worker) = ThreadPool::worker() else {
            ris_error::panic!("attempted to wait on a non worker")
        };

        loop {
            match unsafe {self.receiver.take()} {
                Some(output) => return output,
                None => {
                    let job_was_run = worker.run_pending_job();
                    if !job_was_run {
                        std::thread::yield_now();
                    }
                }
            }
        }
    }
}


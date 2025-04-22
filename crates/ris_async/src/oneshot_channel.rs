use std::cell::UnsafeCell;
use std::mem::MaybeUninit;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;

use crate::ThreadPool;

#[derive(Debug)]
struct OneshotChannel<T> {
    ready: AtomicBool,
    data: UnsafeCell<MaybeUninit<T>>,
}

#[derive(Debug)]
pub struct OneshotSender<T> {
    channel: Arc<OneshotChannel<T>>,
}

#[derive(Debug)]
pub struct OneshotReceiver<T> {
    channel: Arc<OneshotChannel<T>>,
}

unsafe impl<T> Sync for OneshotChannel<T> where T: Send {}

pub fn oneshot_channel<T>() -> (OneshotSender<T>, OneshotReceiver<T>) {
    let channel = Arc::new(OneshotChannel{
        ready: AtomicBool::new(false),
        data: UnsafeCell::new(MaybeUninit::uninit()),
    });

    let sender = OneshotSender {channel: channel.clone()};
    let receiver = OneshotReceiver { channel };

    (sender, receiver)
}

impl<T> OneshotSender<T> {
    pub fn send(self, value: T) {
        unsafe { (*self.channel.data.get()).write(value) };
        self.channel.ready.store(true, Ordering::Release);
    }
}

impl<T> OneshotReceiver<T> {
    pub fn receive(mut self) -> Result<T, Self> {
        match self.take() {
            Some(value) => Ok(value),
            None => Err(self),
        }
    }

    pub fn take(&mut self) -> Option<T> {
        if self.channel.ready.swap(false, Ordering::Acquire) {
            let output = unsafe { (*self.channel.data.get()).assume_init_read() };
            Some(output)
        } else {
            None
        }
    }

    pub fn wait(mut self) -> T {
        loop {
            match self.take() {
                Some(output) => return output,
                None => {
                    if !ThreadPool::run_pending_job() {
                        std::thread::yield_now();
                    }
                },
            }
        }
    }
}

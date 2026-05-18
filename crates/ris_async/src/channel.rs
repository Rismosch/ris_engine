use std::cell::UnsafeCell;
use std::ffi::c_void;
use std::mem::MaybeUninit;
use std::ptr::NonNull;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

use crate::ThreadPool;

// structs
#[derive(Debug)]
struct SingleUseChannel<T> {
    ready: AtomicBool,
    data: UnsafeCell<MaybeUninit<T>>,
}

#[derive(Debug)]
pub struct SingleUseSender<T> {
    channel: Arc<SingleUseChannel<T>>,
}

#[derive(Debug)]
pub struct SingleUseReceiver<T> {
    channel: Arc<SingleUseChannel<T>>,
}

unsafe impl<T> Sync for SingleUseChannel<T> where T: Send {}

#[derive(Debug)]
struct UnsafeChannel {
    ready: AtomicBool,
    p_data: *mut c_void,
}

#[derive(Debug)]
pub struct UnsafeSender {
    channel: Arc<UnsafeChannel>,
}

#[derive(Debug)]
pub struct UnsafeReceiver {
    channel: Arc<UnsafeChannel>,
}

// constructor
pub fn single_use_channel<T>() -> (SingleUseSender<T>, SingleUseReceiver<T>) {
    let channel = Arc::new(SingleUseChannel {
        ready: AtomicBool::new(false),
        data: UnsafeCell::new(MaybeUninit::uninit()),
    });

    let sender = SingleUseSender {
        channel: channel.clone(),
    };
    let receiver = SingleUseReceiver { channel };

    (sender, receiver)
}

pub fn unsafe_channel<T>(p_data: NonNull<T>) -> (UnsafeSender, UnsafeReceiver) {
    let channel = Arc::new(UnsafeChannel {
        ready: AtomicBool::new(false),
        p_data: p_data.as_ptr() as *mut c_void,
    });

    let sender = UnsafeSender {
        channel: channel.clone(),
    };
    let receiver = UnsafeReceiver {channel};

    (sender, receiver)
}

// functions
impl<T> SingleUseSender<T> {
    pub fn send(self, value: T) {
        unsafe { (*self.channel.data.get()).write(value) };
        self.channel.ready.store(true, Ordering::Release);
    }
}

impl UnsafeSender {
    pub fn as_mut(&self) -> *mut c_void {
        self.channel.p_data
    }

    pub unsafe fn assume_init(self) {
        self.channel.ready.store(true, Ordering::Release);
    }
}

impl<T> SingleUseReceiver<T> {
    pub fn with_value(value: T) -> Self {
        let channel = Arc::new(SingleUseChannel {
            ready: AtomicBool::new(true),
            data: UnsafeCell::new(MaybeUninit::new(value)),
        });

        Self { channel }
    }

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
                }
            }
        }
    }
}

impl UnsafeReceiver {
    pub unsafe fn is_init(&mut self) -> bool {
        self.channel.ready.swap(false, Ordering::Acquire)
    }
}

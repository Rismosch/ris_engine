use std::{sync::atomic::{AtomicBool, Ordering}, thread};

pub struct AtomicLock<'a>{
    lock: &'a AtomicBool
}

impl<'a> AtomicLock <'a>{
    pub fn wait_and_lock(lock: &'a mut  AtomicBool) -> Self {
        while lock.compare_exchange_weak(false, true, Ordering::SeqCst, Ordering::SeqCst).is_err() {
            thread::yield_now();
        }

        AtomicLock { lock }
    }
}

impl<'a> Drop for AtomicLock<'a> {
    fn drop(&mut self) {
        self.lock.store(false, Ordering::SeqCst)
    }
}
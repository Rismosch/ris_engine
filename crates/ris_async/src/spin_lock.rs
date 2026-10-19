// API design from "Rust Atomics and Locks" from Mara Bos:
// https://mara.nl/atomics/building-spinlock.html
//
// Lock strategy from David Álvarez Rosa's SpinLockV4:
// https://david.alvarezrosa.com/posts/optimizing-a-spin-lock/

use std::cell::UnsafeCell;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

pub struct SpinLock<T> {
    locked: AtomicBool,
    value: UnsafeCell<T>,
}

pub struct SpinLockGuard<'a, T> {
    lock: &'a SpinLock<T>,
}

impl<T: Default> Default for SpinLock<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<T> SpinLock<T> {
    pub const fn new(value: T) -> Self {
        Self {
            locked: AtomicBool::new(false),
            value: UnsafeCell::new(value),
        }
    }

    pub fn lock(&self) -> SpinLockGuard<'_, T> {
        let mut backoff = 1;
        while self.locked.swap(true, Ordering::Acquire) {
            loop {
                for _ in 0..backoff {
                    std::thread::yield_now();
                }

                backoff = if backoff < 64 {
                    backoff << 1
                } else {
                    64
                };

                if self.locked.load(Ordering::Relaxed) {
                    continue;
                } else {
                    break;
                }
            }
        }

        SpinLockGuard { lock: self }
    }
}

impl<T> Drop for SpinLockGuard<'_, T> {
    fn drop(&mut self) {
        self.lock.locked.store(false, Ordering::Release);
    }
}

impl<T> std::ops::Deref for SpinLockGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.lock.value.get() }
    }
}

impl<T> std::ops::DerefMut for SpinLockGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.lock.value.get() }
    }
}

unsafe impl<T> Sync for SpinLock<T> where T: Send {}

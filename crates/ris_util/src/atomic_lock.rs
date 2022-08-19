use std::{
    sync::atomic::{AtomicBool, Ordering},
    thread,
};

/// Mutex-like utility, mainly used to execute tests synchronously.
pub struct AtomicLock<'a> {
    lock: &'a AtomicBool,
}

impl<'a> AtomicLock<'a> {
    pub fn wait_and_lock(lock: &'a mut AtomicBool) -> Self {
        let result = AtomicLock { lock };

        result.lock();

        result
    }

    fn lock(&self) {
        while self
            .lock
            .compare_exchange_weak(false, true, Ordering::SeqCst, Ordering::SeqCst)
            .is_err()
        {
            thread::yield_now();
        }
    }

    fn unlock(&self) {
        self.lock.store(false, Ordering::SeqCst)
    }
}

impl<'a> Drop for AtomicLock<'a> {
    fn drop(&mut self) {
        self.unlock()
    }
}

#[cfg(test)]
mod examples {
    use crate::atomic_lock::AtomicLock;
    use std::sync::atomic::AtomicBool;

    // Globally shared `AtomicBool`, to sync tests.
    static mut LOCK: AtomicBool = AtomicBool::new(false);

    #[test]
    fn single_threaded_test_one() {
        // Lock the `AtomicBool` and prevent other tests from executing.
        // If another test is holding the lock, `wait_and_lock()` spinlocks until it aquired the lock.
        let lock = AtomicLock::wait_and_lock(unsafe { &mut LOCK });

        // do your testing...
        assert!(true);

        // Calling `drop()` manually on the lock, prevents it from being dropped early.
        // Thus, the lock is held up to this point.
        drop(lock)
    }

    #[test]
    #[should_panic]
    fn single_threaded_test_two() {
        let lock = AtomicLock::wait_and_lock(unsafe { &mut LOCK });

        assert!(false);

        drop(lock)
    }
}

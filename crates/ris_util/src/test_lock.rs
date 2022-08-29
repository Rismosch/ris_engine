use std::{
    sync::atomic::{AtomicBool, Ordering},
    thread,
};

pub struct TestLock<'a>(&'a AtomicBool);

impl<'a> TestLock<'a> {
    pub fn wait_and_lock(lock: &'a AtomicBool) -> Self {
        while lock
            .compare_exchange_weak(false, true, Ordering::SeqCst, Ordering::SeqCst)
            .is_err()
        {
            thread::yield_now();
        }

        Self(lock)
    }
}

impl<'a> Drop for TestLock<'a> {
    fn drop(&mut self) {
        self.0.store(false, Ordering::SeqCst)
    }
}

#[cfg(test)]
mod examples {
    use std::{sync::atomic::AtomicBool, thread, time::Duration};

    use super::TestLock;

    static mut UNSAFE_SHARED_DATA: String = String::new();
    static LOCK: AtomicBool = AtomicBool::new(false);

    #[test]
    fn test_one() {
        let lock = TestLock::wait_and_lock(&LOCK);

        unsafe {
            UNSAFE_SHARED_DATA = String::from("hoi");
            thread::sleep(Duration::from_millis(1));
            assert_eq!(UNSAFE_SHARED_DATA, "hoi");
        }

        drop(lock)
    }

    #[test]
    fn test_two() {
        let lock = TestLock::wait_and_lock(&LOCK);

        unsafe {
            UNSAFE_SHARED_DATA = String::from("poi");
            assert_eq!(UNSAFE_SHARED_DATA, "poi");
        }

        drop(lock)
    }
}

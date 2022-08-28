#[cfg(test)]
mod examples {
    use std::{sync::Mutex, thread, time::Duration};

    static mut UNSAFE_SHARED_DATA: String = String::new();
    static mut LOCK: Mutex<()> = Mutex::new(());

    #[test]
    fn test_one() {
        // You must give the variable a proper name, besides just `_`.
        // Otherwise, Rust drops the lock too early.
        let _lock = unsafe {LOCK.lock().unwrap()};

        unsafe {
            UNSAFE_SHARED_DATA = String::from("hoi");
            thread::sleep(Duration::from_millis(1));
            assert_eq!(UNSAFE_SHARED_DATA, "hoi");
        }
    }

    #[test]
    fn test_two() {
        let _lock = unsafe {LOCK.lock().unwrap()};

        unsafe {
            UNSAFE_SHARED_DATA = String::from("poi");
            assert_eq!(UNSAFE_SHARED_DATA, "poi");
        }
    }
}

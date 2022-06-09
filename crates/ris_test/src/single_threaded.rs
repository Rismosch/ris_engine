use std::sync::atomic::{AtomicBool, Ordering};
static mut THREAD_BLOCKED: AtomicBool = AtomicBool::new(false);
pub fn test_single_threaded(test: fn() -> ()) {
    loop {
        let result = unsafe {
            THREAD_BLOCKED.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
        };

        if result.is_err() {
            std::thread::sleep(std::time::Duration::from_millis(1));
            continue;
        }

        let result = std::panic::catch_unwind(test);

        let _ = unsafe { THREAD_BLOCKED.swap(false, Ordering::Relaxed) };

        assert!(result.is_ok());

        break;
    }
}

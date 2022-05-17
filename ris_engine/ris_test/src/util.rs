pub fn retry(count: usize, test: fn() -> ()) {
    for _ in 0..count - 1 {
        let result = std::panic::catch_unwind(test);

        if result.is_ok() {
            return;
        }
    }

    test();
}

pub fn repeat(count: usize, test: fn() -> ()) {
    for _ in 0..count {
        test();
    }
}

pub fn wrap<T>(teardown: fn() -> (), test: T)
where
    T: FnOnce() + std::panic::UnwindSafe,
{
    let result = std::panic::catch_unwind(test);
    teardown();
    assert!(result.is_ok());
}

use std::sync::atomic::{AtomicBool, Ordering};
static mut CALL_COUNT: AtomicBool = AtomicBool::new(false);
pub fn single_threaded(test: fn() -> ()) {
    loop {
        let result = unsafe {
            CALL_COUNT.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
        };

        if result.is_err() {
            std::thread::sleep(std::time::Duration::from_millis(10));
            continue;
        }

        let result = std::panic::catch_unwind(test);

        let _ = unsafe { CALL_COUNT.swap(false, Ordering::Relaxed) };

        assert!(result.is_ok());

        break;
    }
}

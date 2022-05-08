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

use std::sync::atomic::{AtomicUsize, Ordering};
static mut CALL_COUNT: AtomicUsize = AtomicUsize::new(0);
pub fn single_threaded(test: fn() -> ()) {
    unsafe {
        loop {
            let result = CALL_COUNT.compare_exchange(
                0,
                1,
                Ordering::Acquire,
                Ordering::Relaxed
            );
            
            if result.is_err() {
                std::thread::sleep(std::time::Duration::from_millis(10));
                continue;
            }

            let result = std::panic::catch_unwind(test);

            let _ = CALL_COUNT.swap(0, Ordering::Relaxed);

            assert!(result.is_ok());

            break;
        }
    }
}
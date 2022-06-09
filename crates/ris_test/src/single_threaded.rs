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

#[cfg(test)]
mod tests {
    use super::*;

    static mut SINGLE_THREADED_SUCCEED_VEC: Vec<char> = Vec::new();
    #[test]
    fn should_run_tests_sequentially() {
        unsafe {
            let mut handles = Vec::new();

            handles.push(std::thread::spawn(|| {
                test_single_threaded(|| {
                    SINGLE_THREADED_SUCCEED_VEC.push('a');
                    std::thread::sleep(std::time::Duration::from_millis(400));
                    SINGLE_THREADED_SUCCEED_VEC.push('b');
                })
            }));
            std::thread::sleep(std::time::Duration::from_millis(10));
            handles.push(std::thread::spawn(|| {
                test_single_threaded(|| {
                    SINGLE_THREADED_SUCCEED_VEC.push('a');
                    std::thread::sleep(std::time::Duration::from_millis(300));
                    SINGLE_THREADED_SUCCEED_VEC.push('b');
                })
            }));
            std::thread::sleep(std::time::Duration::from_millis(10));
            handles.push(std::thread::spawn(|| {
                test_single_threaded(|| {
                    SINGLE_THREADED_SUCCEED_VEC.push('a');
                    std::thread::sleep(std::time::Duration::from_millis(200));
                    SINGLE_THREADED_SUCCEED_VEC.push('b');
                })
            }));
            std::thread::sleep(std::time::Duration::from_millis(10));
            handles.push(std::thread::spawn(|| {
                test_single_threaded(|| {
                    SINGLE_THREADED_SUCCEED_VEC.push('a');
                    std::thread::sleep(std::time::Duration::from_millis(100));
                    SINGLE_THREADED_SUCCEED_VEC.push('b');
                })
            }));

            for handle in handles {
                handle.join().unwrap();
            }

            assert_eq!(SINGLE_THREADED_SUCCEED_VEC.len(), 8);
            assert_eq!(SINGLE_THREADED_SUCCEED_VEC[0], 'a');
            assert_eq!(SINGLE_THREADED_SUCCEED_VEC[1], 'b');
            assert_eq!(SINGLE_THREADED_SUCCEED_VEC[2], 'a');
            assert_eq!(SINGLE_THREADED_SUCCEED_VEC[3], 'b');
            assert_eq!(SINGLE_THREADED_SUCCEED_VEC[4], 'a');
            assert_eq!(SINGLE_THREADED_SUCCEED_VEC[5], 'b');
            assert_eq!(SINGLE_THREADED_SUCCEED_VEC[6], 'a');
            assert_eq!(SINGLE_THREADED_SUCCEED_VEC[7], 'b');
        }
    }

    static mut SINGLE_THREADED_FAIL_VEC: Vec<char> = Vec::new();
    static mut SINGLE_THREADED_FAIL_RESULTS: Vec<Option<bool>> = Vec::new();
    #[test]
    fn should_panic() {
        unsafe {
            let mut handles = Vec::new();
            SINGLE_THREADED_FAIL_RESULTS.push(None);
            SINGLE_THREADED_FAIL_RESULTS.push(None);
            SINGLE_THREADED_FAIL_RESULTS.push(None);
            SINGLE_THREADED_FAIL_RESULTS.push(None);

            handles.push(std::thread::spawn(move || {
                let result = std::panic::catch_unwind(|| {
                    test_single_threaded(|| {
                        SINGLE_THREADED_FAIL_VEC.push('a');
                        std::thread::sleep(std::time::Duration::from_millis(400));
                        SINGLE_THREADED_FAIL_VEC.push('b');
                        panic!();
                    })
                });
                SINGLE_THREADED_FAIL_RESULTS[0] = Some(result.is_ok());
            }));
            std::thread::sleep(std::time::Duration::from_millis(10));
            handles.push(std::thread::spawn(move || {
                let result = std::panic::catch_unwind(|| {
                    test_single_threaded(|| {
                        SINGLE_THREADED_FAIL_VEC.push('a');
                        std::thread::sleep(std::time::Duration::from_millis(300));
                        SINGLE_THREADED_FAIL_VEC.push('b');
                    })
                });
                SINGLE_THREADED_FAIL_RESULTS[1] = Some(result.is_ok());
            }));
            std::thread::sleep(std::time::Duration::from_millis(10));
            handles.push(std::thread::spawn(move || {
                let result = std::panic::catch_unwind(|| {
                    test_single_threaded(|| {
                        SINGLE_THREADED_FAIL_VEC.push('a');
                        std::thread::sleep(std::time::Duration::from_millis(200));
                        SINGLE_THREADED_FAIL_VEC.push('b');
                        panic!();
                    })
                });
                SINGLE_THREADED_FAIL_RESULTS[2] = Some(result.is_ok());
            }));
            std::thread::sleep(std::time::Duration::from_millis(10));
            handles.push(std::thread::spawn(move || {
                let result = std::panic::catch_unwind(|| {
                    test_single_threaded(|| {
                        SINGLE_THREADED_FAIL_VEC.push('a');
                        std::thread::sleep(std::time::Duration::from_millis(100));
                        SINGLE_THREADED_FAIL_VEC.push('b');
                    })
                });
                SINGLE_THREADED_FAIL_RESULTS[3] = Some(result.is_ok());
            }));

            for handle in handles {
                handle.join().unwrap();
            }

            assert_eq!(SINGLE_THREADED_FAIL_VEC.len(), 8);
            assert_eq!(SINGLE_THREADED_FAIL_VEC[0], 'a');
            assert_eq!(SINGLE_THREADED_FAIL_VEC[1], 'b');
            assert_eq!(SINGLE_THREADED_FAIL_VEC[2], 'a');
            assert_eq!(SINGLE_THREADED_FAIL_VEC[3], 'b');
            assert_eq!(SINGLE_THREADED_FAIL_VEC[4], 'a');
            assert_eq!(SINGLE_THREADED_FAIL_VEC[5], 'b');
            assert_eq!(SINGLE_THREADED_FAIL_VEC[6], 'a');
            assert_eq!(SINGLE_THREADED_FAIL_VEC[7], 'b');

            assert_eq!(SINGLE_THREADED_FAIL_RESULTS.len(), 4);
            assert_eq!(SINGLE_THREADED_FAIL_RESULTS[0], Some(false));
            assert_eq!(SINGLE_THREADED_FAIL_RESULTS[1], Some(true));
            assert_eq!(SINGLE_THREADED_FAIL_RESULTS[2], Some(false));
            assert_eq!(SINGLE_THREADED_FAIL_RESULTS[3], Some(true));
        }
    }
}

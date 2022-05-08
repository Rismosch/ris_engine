use ris_test::util::*;

static mut RETRY_SHOULD_SUCCEED_COUNT: i32 = 0;
#[test]
fn retry_should_succeed() {
    unsafe {
        RETRY_SHOULD_SUCCEED_COUNT = 0;

        retry(10, || {
            RETRY_SHOULD_SUCCEED_COUNT += 1;
            if RETRY_SHOULD_SUCCEED_COUNT < 5 {
                panic!();
            }
        });

        assert_eq!(RETRY_SHOULD_SUCCEED_COUNT, 5);
    }
}

static mut RETRY_SHOULD_FAIL_COUNT: i32 = 0;
#[test]
fn retry_should_fail() {
    unsafe {
        RETRY_SHOULD_FAIL_COUNT = 0;

        let result = std::panic::catch_unwind(|| {
            retry(10, || {
                RETRY_SHOULD_FAIL_COUNT += 1;
                panic!();
            })
        });

        assert_eq!(RETRY_SHOULD_FAIL_COUNT, 10);
        assert!(result.is_err());
    }
}

static mut REPEAT_SHOULD_SUCCEED_COUNT: i32 = 0;
#[test]
fn repeat_should_succeed() {
    unsafe {
        REPEAT_SHOULD_SUCCEED_COUNT = 0;

        repeat(10, || {
            REPEAT_SHOULD_SUCCEED_COUNT += 1;
        });

        assert_eq!(REPEAT_SHOULD_SUCCEED_COUNT, 10);
    }
}

static mut REPEAT_SHOULD_FAIL_COUNT: i32 = 0;
#[test]
fn repeat_should_fail() {
    unsafe {
        REPEAT_SHOULD_FAIL_COUNT = 0;

        let result = std::panic::catch_unwind(|| {
            repeat(10, || {
                REPEAT_SHOULD_FAIL_COUNT += 1;
                if REPEAT_SHOULD_FAIL_COUNT >= 5 {
                    panic!();
                }
            });
        });

        assert_eq!(REPEAT_SHOULD_FAIL_COUNT, 5);
        assert!(result.is_err());
    }
}

static mut WRAP_SHOULD_SUCCEED_VEC: Vec<i32> = Vec::new();
#[test]
fn wrap_should_succeed() {
    unsafe {
        WRAP_SHOULD_SUCCEED_VEC = Vec::new();

        wrap(
            || WRAP_SHOULD_SUCCEED_VEC.push(2),
            || WRAP_SHOULD_SUCCEED_VEC.push(1),
        );

        assert_eq!(WRAP_SHOULD_SUCCEED_VEC.len(), 2);
        assert_eq!(WRAP_SHOULD_SUCCEED_VEC[0], 1);
        assert_eq!(WRAP_SHOULD_SUCCEED_VEC[1], 2);
    }
}

static mut WRAP_SHOULD_FAIL_VEC: Vec<i32> = Vec::new();
#[test]
fn wrap_should_fail() {
    unsafe {
        WRAP_SHOULD_FAIL_VEC = Vec::new();

        let result = std::panic::catch_unwind(|| {
            wrap(
                || WRAP_SHOULD_FAIL_VEC.push(2),
                || {
                    WRAP_SHOULD_FAIL_VEC.push(1);
                    panic!();
                },
            );
        });

        assert_eq!(WRAP_SHOULD_FAIL_VEC.len(), 2);
        assert_eq!(WRAP_SHOULD_FAIL_VEC[0], 1);
        assert_eq!(WRAP_SHOULD_FAIL_VEC[1], 2);
        assert!(result.is_err());
    }
}

static mut SINGLE_THREADED_SUCCEED_VEC: Vec<char> = Vec::new();
#[test]
fn single_threaded_should_run_tests_sequentially() {
    unsafe {
        let mut handles = Vec::new();

        handles.push(std::thread::spawn(|| {
            single_threaded(|| {
                SINGLE_THREADED_SUCCEED_VEC.push('a');
                std::thread::sleep(std::time::Duration::from_millis(400));
                SINGLE_THREADED_SUCCEED_VEC.push('b');
            })
        }));
        std::thread::sleep(std::time::Duration::from_millis(10));
        handles.push(std::thread::spawn(|| {
            single_threaded(|| {
                SINGLE_THREADED_SUCCEED_VEC.push('a');
                std::thread::sleep(std::time::Duration::from_millis(300));
                SINGLE_THREADED_SUCCEED_VEC.push('b');
            })
        }));
        std::thread::sleep(std::time::Duration::from_millis(10));
        handles.push(std::thread::spawn(|| {
            single_threaded(|| {
                SINGLE_THREADED_SUCCEED_VEC.push('a');
                std::thread::sleep(std::time::Duration::from_millis(200));
                SINGLE_THREADED_SUCCEED_VEC.push('b');
            })
        }));
        std::thread::sleep(std::time::Duration::from_millis(10));
        handles.push(std::thread::spawn(|| {
            single_threaded(|| {
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
fn single_threaded_should_panic() {
    unsafe {
        let mut handles = Vec::new();
        SINGLE_THREADED_FAIL_RESULTS.push(None);
        SINGLE_THREADED_FAIL_RESULTS.push(None);
        SINGLE_THREADED_FAIL_RESULTS.push(None);
        SINGLE_THREADED_FAIL_RESULTS.push(None);

        handles.push(std::thread::spawn(move || {
            let result = std::panic::catch_unwind(|| {
                single_threaded(|| {
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
                single_threaded(|| {
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
                single_threaded(|| {
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
                single_threaded(|| {
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

        assert_eq!(SINGLE_THREADED_SUCCEED_VEC.len(), 8);
        assert_eq!(SINGLE_THREADED_SUCCEED_VEC[0], 'a');
        assert_eq!(SINGLE_THREADED_SUCCEED_VEC[1], 'b');
        assert_eq!(SINGLE_THREADED_SUCCEED_VEC[2], 'a');
        assert_eq!(SINGLE_THREADED_SUCCEED_VEC[3], 'b');
        assert_eq!(SINGLE_THREADED_SUCCEED_VEC[4], 'a');
        assert_eq!(SINGLE_THREADED_SUCCEED_VEC[5], 'b');
        assert_eq!(SINGLE_THREADED_SUCCEED_VEC[6], 'a');
        assert_eq!(SINGLE_THREADED_SUCCEED_VEC[7], 'b');

        assert_eq!(SINGLE_THREADED_FAIL_RESULTS.len(), 4);
        assert_eq!(SINGLE_THREADED_FAIL_RESULTS[0], Some(false));
        assert_eq!(SINGLE_THREADED_FAIL_RESULTS[1], Some(true));
        assert_eq!(SINGLE_THREADED_FAIL_RESULTS[2], Some(false));
        assert_eq!(SINGLE_THREADED_FAIL_RESULTS[3], Some(true));
    }
}

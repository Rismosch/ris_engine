use ris_test::{single_thread_test::execute_single_thread_test, test::test};

static mut SUCCEED_VEC: Vec<char> = Vec::new();
#[test]
fn should_succeed() {
    unsafe {
        SUCCEED_VEC = Vec::new();

        let mut handles = Vec::new();

        handles.push(std::thread::spawn(|| {
            execute_single_thread_test(|| {
                SUCCEED_VEC.push('a');
                std::thread::sleep(std::time::Duration::from_millis(40));
                SUCCEED_VEC.push('b');
            })
        }));
        std::thread::sleep(std::time::Duration::from_millis(10));
        handles.push(std::thread::spawn(|| {
            execute_single_thread_test(|| {
                SUCCEED_VEC.push('a');
                std::thread::sleep(std::time::Duration::from_millis(30));
                SUCCEED_VEC.push('b');
            })
        }));
        std::thread::sleep(std::time::Duration::from_millis(10));
        handles.push(std::thread::spawn(|| {
            execute_single_thread_test(|| {
                SUCCEED_VEC.push('a');
                std::thread::sleep(std::time::Duration::from_millis(20));
                SUCCEED_VEC.push('b');
            })
        }));
        std::thread::sleep(std::time::Duration::from_millis(10));
        handles.push(std::thread::spawn(|| {
            execute_single_thread_test(|| {
                SUCCEED_VEC.push('a');
                std::thread::sleep(std::time::Duration::from_millis(10));
                SUCCEED_VEC.push('b');
            })
        }));

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(SUCCEED_VEC.len(), 8);
        assert_eq!(SUCCEED_VEC[0], 'a');
        assert_eq!(SUCCEED_VEC[1], 'b');
        assert_eq!(SUCCEED_VEC[2], 'a');
        assert_eq!(SUCCEED_VEC[3], 'b');
        assert_eq!(SUCCEED_VEC[4], 'a');
        assert_eq!(SUCCEED_VEC[5], 'b');
        assert_eq!(SUCCEED_VEC[6], 'a');
        assert_eq!(SUCCEED_VEC[7], 'b');
    }
}

static mut FAIL_VEC: Vec<char> = Vec::new();
static mut FAIL_RESULTS: Vec<Option<bool>> = Vec::new();
#[test]
fn should_fail() {
    unsafe {
        let mut handles = Vec::new();
        FAIL_VEC = Vec::new();
        FAIL_RESULTS = Vec::new();
        FAIL_RESULTS.push(None);
        FAIL_RESULTS.push(None);
        FAIL_RESULTS.push(None);
        FAIL_RESULTS.push(None);

        handles.push(std::thread::spawn(move || {
            let result = std::panic::catch_unwind(|| {
                test().single_thread().run(|| {
                    FAIL_VEC.push('a');
                    std::thread::sleep(std::time::Duration::from_millis(40));
                    FAIL_VEC.push('b');
                    panic!()
                })
            });
            FAIL_RESULTS[0] = Some(result.is_ok());
        }));
        std::thread::sleep(std::time::Duration::from_millis(10));
        handles.push(std::thread::spawn(move || {
            let result = std::panic::catch_unwind(|| {
                execute_single_thread_test(|| {
                    FAIL_VEC.push('a');
                    std::thread::sleep(std::time::Duration::from_millis(30));
                    FAIL_VEC.push('b');
                })
            });
            FAIL_RESULTS[1] = Some(result.is_ok());
        }));
        std::thread::sleep(std::time::Duration::from_millis(10));
        handles.push(std::thread::spawn(move || {
            let result = std::panic::catch_unwind(|| {
                execute_single_thread_test(|| {
                    FAIL_VEC.push('a');
                    std::thread::sleep(std::time::Duration::from_millis(20));
                    FAIL_VEC.push('b');
                    panic!()
                })
            });
            FAIL_RESULTS[2] = Some(result.is_ok());
        }));
        std::thread::sleep(std::time::Duration::from_millis(10));
        handles.push(std::thread::spawn(move || {
            let result = std::panic::catch_unwind(|| {
                execute_single_thread_test(|| {
                    FAIL_VEC.push('a');
                    std::thread::sleep(std::time::Duration::from_millis(10));
                    FAIL_VEC.push('b');
                })
            });
            FAIL_RESULTS[3] = Some(result.is_ok());
        }));

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(FAIL_VEC.len(), 8);
        assert_eq!(FAIL_VEC[0], 'a');
        assert_eq!(FAIL_VEC[1], 'b');
        assert_eq!(FAIL_VEC[2], 'a');
        assert_eq!(FAIL_VEC[3], 'b');
        assert_eq!(FAIL_VEC[4], 'a');
        assert_eq!(FAIL_VEC[5], 'b');
        assert_eq!(FAIL_VEC[6], 'a');
        assert_eq!(FAIL_VEC[7], 'b');

        assert_eq!(FAIL_RESULTS.len(), 4);
        assert_eq!(FAIL_RESULTS[0], Some(false));
        assert_eq!(FAIL_RESULTS[1], Some(true));
        assert_eq!(FAIL_RESULTS[2], Some(false));
        assert_eq!(FAIL_RESULTS[3], Some(true));
    }
}

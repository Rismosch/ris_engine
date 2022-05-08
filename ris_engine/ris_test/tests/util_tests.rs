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

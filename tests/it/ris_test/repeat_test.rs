use ris_test::repeat_test::{execute_repeat_test, RepeatKind, RepeatTest};

static mut SUCCEED_REPEAT_CALLS: i32 = 0;
#[test]
fn should_succeed_on_repeat() {
    unsafe {
        SUCCEED_REPEAT_CALLS = 0;
    }

    execute_repeat_test(10, RepeatKind::Repeat, || unsafe {
        SUCCEED_REPEAT_CALLS += 1
    });

    let calls = unsafe { SUCCEED_REPEAT_CALLS };

    assert_eq!(calls, 10);
}

static mut FAIL_REPEAT_CALLS: i32 = 0;
#[test]
fn should_fail_on_repeat() {
    unsafe {
        FAIL_REPEAT_CALLS = 0;
    }

    let result = std::panic::catch_unwind(|| {
        execute_repeat_test(10, RepeatKind::Repeat, || unsafe {
            FAIL_REPEAT_CALLS += 1;
            if FAIL_REPEAT_CALLS >= 5 {
                panic!();
            }
        });
    });

    assert!(result.is_err());

    let calls = unsafe { FAIL_REPEAT_CALLS };

    assert_eq!(calls, 5);
}

static mut SUCCEED_FIRST_TRY_CALLS: i32 = 0;
#[test]
fn should_succeed_on_first_try() {
    unsafe {
        SUCCEED_FIRST_TRY_CALLS = 0;
    }

    execute_repeat_test(10, RepeatKind::Retry, || unsafe {
        SUCCEED_FIRST_TRY_CALLS += 1;
    });

    let calls = unsafe { SUCCEED_FIRST_TRY_CALLS };

    assert_eq!(calls, 1);
}

static mut SUCCEED_RETRY_CALLS: i32 = 0;
#[test]
fn should_succeed_somewhere_after_first_try() {
    unsafe {
        SUCCEED_RETRY_CALLS = 0;
    }

    execute_repeat_test(10, RepeatKind::Retry, || unsafe {
        SUCCEED_RETRY_CALLS += 1;
        if SUCCEED_RETRY_CALLS < 5 {
            panic!()
        }
    });

    let calls = unsafe { SUCCEED_RETRY_CALLS };

    assert_eq!(calls, 5);
}

static mut FAIL_RETRY_CALLS: i32 = 0;
#[test]
fn should_fail_when_every_retry_fails() {
    unsafe {
        FAIL_RETRY_CALLS = 0;
    }

    let result = std::panic::catch_unwind(|| {
        execute_repeat_test(10, RepeatKind::Retry, || unsafe {
            FAIL_RETRY_CALLS += 1;
            panic!()
        });
    });

    assert!(result.is_err());

    let calls = unsafe { FAIL_RETRY_CALLS };

    assert_eq!(calls, 10);
}

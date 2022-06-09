use ris_test::repeat::test_repeat;

static mut REPEAT_SHOULD_SUCCEED_VEC: Vec<usize> = Vec::new();
#[test]
fn should_succeed() {
    unsafe {
        REPEAT_SHOULD_SUCCEED_VEC = Vec::new();

        test_repeat(10, |index| {
            REPEAT_SHOULD_SUCCEED_VEC.push(index);
        });

        assert_eq!(REPEAT_SHOULD_SUCCEED_VEC, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }
}

static mut REPEAT_SHOULD_FAIL_VEC: Vec<usize> = Vec::new();
#[test]
fn should_fail() {
    unsafe {
        REPEAT_SHOULD_FAIL_VEC = Vec::new();

        let result = std::panic::catch_unwind(|| {
            test_repeat(10, |index| {
                REPEAT_SHOULD_FAIL_VEC.push(index);
                if REPEAT_SHOULD_FAIL_VEC.len() >= 5 {
                    panic!();
                }
            });
        });

        assert_eq!(REPEAT_SHOULD_FAIL_VEC, [0, 1, 2, 3, 4]);
        assert!(result.is_err());
    }
}

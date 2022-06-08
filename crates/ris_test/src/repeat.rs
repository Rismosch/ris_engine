pub fn repeat(count: usize, test: fn() -> ()) {
    for _ in 0..count {
        test();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
pub trait ITestContext {
    fn setup() -> Self;
    fn teardown(&mut self);
}

pub fn test_harness<TContext>(test: Box<dyn FnOnce(&mut TContext) + std::panic::UnwindSafe>)
where
    TContext: ITestContext + std::panic::RefUnwindSafe + std::panic::UnwindSafe,
{
    let result;

    let mut test_context = TContext::setup();
    let raw_test_context = &mut test_context as *mut TContext;

    unsafe {
        result = std::panic::catch_unwind(move || {
            test(raw_test_context.as_mut().unwrap());
        });

        raw_test_context.as_mut().unwrap().teardown();
    }
    assert!(result.is_ok());
}

#[cfg(test)]
mod tests {
    use super::*;

    static mut HARNESS_SHOULD_SUCCEED_VEC: Vec<i32> = Vec::new();
    #[test]
    fn should_succeed() {
        struct TestContext {
            value: i32,
        }

        impl ITestContext for TestContext {
            fn setup() -> Self {
                unsafe { HARNESS_SHOULD_SUCCEED_VEC.push(1) };
                TestContext { value: 0 }
            }

            fn teardown(&mut self) {
                assert_eq!(self.value, 42);
                unsafe { HARNESS_SHOULD_SUCCEED_VEC.push(3) };
            }
        }

        unsafe {
            HARNESS_SHOULD_SUCCEED_VEC = Vec::new();

            let result = std::panic::catch_unwind(|| {
                test_harness::<TestContext>(Box::new(|context| {
                    context.value = 42;
                    HARNESS_SHOULD_SUCCEED_VEC.push(2)
                }));
            });

            assert!(result.is_ok());

            assert_eq!(HARNESS_SHOULD_SUCCEED_VEC.len(), 3);
            assert_eq!(HARNESS_SHOULD_SUCCEED_VEC[0], 1);
            assert_eq!(HARNESS_SHOULD_SUCCEED_VEC[1], 2);
            assert_eq!(HARNESS_SHOULD_SUCCEED_VEC[2], 3);
        }
    }

    static mut HARNESS_SHOULD_FAIL_IN_SETUP_VEC: Vec<i32> = Vec::new();
    #[test]
    fn should_fail_in_setup() {
        struct TestContext;

        impl ITestContext for TestContext {
            fn setup() -> Self {
                unsafe { HARNESS_SHOULD_FAIL_IN_SETUP_VEC.push(1) };
                panic!();
            }

            fn teardown(&mut self) {
                unsafe { HARNESS_SHOULD_FAIL_IN_SETUP_VEC.push(3) };
            }
        }

        unsafe {
            HARNESS_SHOULD_FAIL_IN_SETUP_VEC = Vec::new();

            let result = std::panic::catch_unwind(|| {
                test_harness::<TestContext>(Box::new(|_| {
                    HARNESS_SHOULD_FAIL_IN_SETUP_VEC.push(2);
                }));
            });

            assert!(result.is_err());

            assert_eq!(HARNESS_SHOULD_FAIL_IN_SETUP_VEC.len(), 1);
            assert_eq!(HARNESS_SHOULD_FAIL_IN_SETUP_VEC[0], 1);
        }
    }

    static mut HARNESS_SHOULD_FAIL_IN_TEST_VEC: Vec<i32> = Vec::new();
    #[test]
    fn should_fail_in_test() {
        struct TestContext;

        impl ITestContext for TestContext {
            fn setup() -> Self {
                unsafe { HARNESS_SHOULD_FAIL_IN_TEST_VEC.push(1) };
                TestContext {}
            }

            fn teardown(&mut self) {
                unsafe { HARNESS_SHOULD_FAIL_IN_TEST_VEC.push(3) };
            }
        }

        unsafe {
            HARNESS_SHOULD_FAIL_IN_TEST_VEC = Vec::new();

            let result = std::panic::catch_unwind(|| {
                test_harness::<TestContext>(Box::new(|_| {
                    HARNESS_SHOULD_FAIL_IN_TEST_VEC.push(2);
                    panic!();
                }));
            });

            assert!(result.is_err());

            assert_eq!(HARNESS_SHOULD_FAIL_IN_TEST_VEC.len(), 3);
            assert_eq!(HARNESS_SHOULD_FAIL_IN_TEST_VEC[0], 1);
            assert_eq!(HARNESS_SHOULD_FAIL_IN_TEST_VEC[1], 2);
            assert_eq!(HARNESS_SHOULD_FAIL_IN_TEST_VEC[2], 3);
        }
    }

    static mut HARNESS_SHOULD_FAIL_IN_TEARDOWN_VEC: Vec<i32> = Vec::new();
    #[test]
    fn should_fail_in_teardown() {
        struct TestContext {}

        impl ITestContext for TestContext {
            fn setup() -> Self {
                unsafe { HARNESS_SHOULD_FAIL_IN_TEARDOWN_VEC.push(1) };
                TestContext {}
            }

            fn teardown(&mut self) {
                unsafe { HARNESS_SHOULD_FAIL_IN_TEARDOWN_VEC.push(3) };
                panic!();
            }
        }

        unsafe {
            HARNESS_SHOULD_FAIL_IN_TEARDOWN_VEC = Vec::new();

            let result = std::panic::catch_unwind(|| {
                test_harness::<TestContext>(Box::new(|_| {
                    HARNESS_SHOULD_FAIL_IN_TEARDOWN_VEC.push(2)
                }));
            });

            assert!(result.is_err());

            assert_eq!(HARNESS_SHOULD_FAIL_IN_TEARDOWN_VEC.len(), 3);
            assert_eq!(HARNESS_SHOULD_FAIL_IN_TEARDOWN_VEC[0], 1);
            assert_eq!(HARNESS_SHOULD_FAIL_IN_TEARDOWN_VEC[1], 2);
            assert_eq!(HARNESS_SHOULD_FAIL_IN_TEARDOWN_VEC[2], 3);
        }
    }
}

pub trait ITestContext {
    fn setup() -> Self;
    fn teardown(&mut self);
}

pub fn harness<TContext>(test: Box<dyn FnOnce(&mut TContext) + std::panic::UnwindSafe>)
where
    TContext: ITestContext + std::panic::RefUnwindSafe + std::panic::UnwindSafe,
{
    let result = std::panic::catch_unwind(move || {
        let mut text_context = TContext::setup();
        test(&mut text_context);
        text_context.teardown();
    });
    assert!(result.is_ok());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(PartialEq, Debug)]
    enum TestSection{
        Setup,
        Test,
        Teardown,
    }

    static mut HARNESS_SHOULD_SUCCEED_VEC: Vec<TestSection> = Vec::new();
    #[test]
    fn harness_should_succeed() {
        struct TestContext{
            value: i32,
        }

        impl ITestContext for TestContext{
            fn setup() -> Self {
                unsafe{HARNESS_SHOULD_SUCCEED_VEC.push(TestSection::Setup)};
                TestContext { value: 0 }
            }

            fn teardown(&mut self) {
                assert_eq!(self.value, 42);
                unsafe{HARNESS_SHOULD_SUCCEED_VEC.push(TestSection::Teardown)};
            }
        }

        unsafe {
            HARNESS_SHOULD_SUCCEED_VEC = Vec::new();

            harness::<TestContext>(Box::new(|context| {
                context.value = 42;
                HARNESS_SHOULD_SUCCEED_VEC.push(TestSection::Test)
            }));

            assert_eq!(HARNESS_SHOULD_SUCCEED_VEC.len(), 3);
            assert_eq!(HARNESS_SHOULD_SUCCEED_VEC[0], TestSection::Setup);
            assert_eq!(HARNESS_SHOULD_SUCCEED_VEC[1], TestSection::Test);
            assert_eq!(HARNESS_SHOULD_SUCCEED_VEC[2], TestSection::Teardown);
        }
    }

    static mut HARNESS_SHOULD_FAIL_VEC: Vec<i32> = Vec::new();
    #[test]
    fn harness_should_fail_in_setup() {
        // unsafe {
        //     HARNESS_SHOULD_FAIL_VEC = Vec::new();

        //     let result = std::panic::catch_unwind(|| {
        //         harness(
        //             || HARNESS_SHOULD_FAIL_VEC.push(2),
        //             || {
        //                 HARNESS_SHOULD_FAIL_VEC.push(1);
        //                 panic!();
        //             },
        //         );
        //     });

        //     assert_eq!(HARNESS_SHOULD_FAIL_VEC.len(), 2);
        //     assert_eq!(HARNESS_SHOULD_FAIL_VEC[0], 1);
        //     assert_eq!(HARNESS_SHOULD_FAIL_VEC[1], 2);
        //     assert!(result.is_err());
        // }
        panic!();
    }

    #[test]
    fn harness_should_fail_in_test() {
        panic!();
    }

    #[test]
    fn harness_should_fail_in_teardown() {
        panic!();
    }
}
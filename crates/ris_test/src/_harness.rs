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

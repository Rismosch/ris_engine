use std::marker::PhantomData;

use crate::icontext::IContext;

pub struct ContextTest<TContext: IContext> {
    phantom_data: PhantomData<TContext>
}

impl<TContext: IContext + std::panic::RefUnwindSafe + std::panic::UnwindSafe> ContextTest<TContext> {
    pub fn new() -> Self {
        ContextTest{ phantom_data: PhantomData::default() }
    }

    pub fn run<TFn: FnMut(&mut TContext) + std::panic::UnwindSafe>(&self, test_fn: TFn) {
        execute_context_test::<TContext, TFn>(test_fn)
    }
}

pub fn execute_context_test<TContext: IContext + std::panic::RefUnwindSafe + std::panic::UnwindSafe, TFn: FnMut(&mut TContext) + std::panic::UnwindSafe>(mut test: TFn) {
    let result;

    let mut context = TContext::setup();
    let raw_context = &mut context as *mut TContext;
    

    unsafe {
        result = std::panic::catch_unwind(move || {
            test(raw_context.as_mut().unwrap());
        });

        raw_context.as_mut().unwrap().teardown();
    }

    assert!(result.is_ok());

}
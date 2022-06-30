use std::marker::PhantomData;

use crate::icontext::IContext;

pub struct ContextTest<TContext: IContext> {
    phantom_data: PhantomData<TContext>
}

impl<TContext: IContext + std::panic::RefUnwindSafe + std::panic::UnwindSafe> ContextTest<TContext> {
    pub fn new() -> Self {
        ContextTest{ phantom_data: PhantomData::default() }
    }

    pub fn run<T: FnOnce(&mut TContext) + std::panic::UnwindSafe>(&self, test_fn: T) {
        let result;

        let mut context = TContext::setup();
        let raw_context = &mut context as *mut TContext;
        

        unsafe {
            result = std::panic::catch_unwind(move || {
                test_fn(raw_context.as_mut().unwrap());
            });

            raw_context.as_mut().unwrap().teardown();
        }

        assert!(result.is_ok());
    }
}
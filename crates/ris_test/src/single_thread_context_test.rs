use std::marker::PhantomData;

use crate::{icontext::IContext, single_thread_test::execute_single_thread_test, context_test::execute_context_test};

pub struct SingleThreadContextTest<TContext: IContext>{
    phantom_data: PhantomData<TContext>
}

impl<TContext: IContext + std::panic::RefUnwindSafe + std::panic::UnwindSafe> SingleThreadContextTest<TContext> {
    pub fn new() -> Self {
        SingleThreadContextTest { phantom_data: PhantomData::default() }
    }

    pub fn run(&self, test_fn: fn(&mut TContext)) {
        execute_single_thread_test(
            || execute_context_test(test_fn)
        );
    }
}


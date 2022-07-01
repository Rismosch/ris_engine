use std::marker::PhantomData;

use crate::{
    context_test::execute_context_test, icontext::IContext,
    single_thread_test::execute_single_thread_test,
};

pub struct SingleThreadContextTest<TContext: IContext> {
    phantom_data: PhantomData<TContext>,
}

impl<TContext: IContext + std::panic::RefUnwindSafe + std::panic::UnwindSafe> Default
    for SingleThreadContextTest<TContext>
{
    fn default() -> Self {
        SingleThreadContextTest {
            phantom_data: PhantomData::default(),
        }
    }
}

impl<TContext: IContext + std::panic::RefUnwindSafe + std::panic::UnwindSafe>
    SingleThreadContextTest<TContext>
{
    pub fn run(&self, test: fn(&mut TContext)) {
        execute_single_thread_test(|| execute_context_test(test));
    }
}

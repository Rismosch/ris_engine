use std::marker::PhantomData;

use crate::{
    context_test::execute_context_test,
    icontext::IContext,
    repeat_test::{execute_repeat_test, RepeatData},
    single_thread_test::execute_single_thread_test,
};

pub struct RepeatSingleThreadContextTest<TContext: IContext> {
    data: RepeatData,
    phantom_data: PhantomData<TContext>,
}

impl<TContext: IContext + std::panic::RefUnwindSafe + std::panic::UnwindSafe>
    RepeatSingleThreadContextTest<TContext>
{
    pub fn new(data: RepeatData) -> Self {
        RepeatSingleThreadContextTest {
            data,
            phantom_data: PhantomData::default(),
        }
    }

    pub fn run<
        TFnMut: FnMut(&mut TContext) + Clone + std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    >(
        &self,
        test: TFnMut,
    ) {
        execute_repeat_test(self.data.count, self.data.kind.clone(), || {
            execute_single_thread_test(|| execute_context_test(test.clone()))
        });
    }
}

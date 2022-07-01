use std::marker::PhantomData;

use crate::{repeat_test::{RepeatData, execute_repeat_test}, icontext::IContext, single_thread_test::execute_single_thread_test, context_test::execute_context_test};


pub struct RepeatSingleThreadContextTest<TContext: IContext> {
    data: RepeatData,
    phantom_data: PhantomData<TContext>,
}

impl<TContext: IContext + std::panic::RefUnwindSafe + std::panic::UnwindSafe> RepeatSingleThreadContextTest<TContext> {
    pub fn new(data: RepeatData) -> Self {
        RepeatSingleThreadContextTest { data, phantom_data: PhantomData::default() }
    }

    pub fn run<TFnMut: FnMut(&mut TContext) + Clone + std::panic::UnwindSafe + std::panic::RefUnwindSafe>(&self, test: TFnMut) {
        execute_repeat_test(self.data.count, self.data.kind.clone(), ||
            execute_single_thread_test(||
                execute_context_test(test.clone())
            )
        );
    }
}
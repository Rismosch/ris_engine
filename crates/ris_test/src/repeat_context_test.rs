use std::marker::PhantomData;

use crate::{icontext::IContext, repeat_test::{RepeatData, execute_repeat_test}, context_test::{execute_context_test, self}};

pub struct RepeatContextTest<TContext: IContext> {
    phantom_data: PhantomData<TContext>,
    data: RepeatData,
}

impl<TContext: IContext + std::panic::RefUnwindSafe + std::panic::UnwindSafe> RepeatContextTest<TContext> {
    pub fn new(data: RepeatData) -> Self {
        RepeatContextTest {phantom_data: PhantomData::default(), data}
    }
    
    pub fn run<TFnMut: FnMut(&mut TContext) + Clone + std::panic::UnwindSafe + std::panic::RefUnwindSafe>(&self, test: TFnMut) {

        execute_repeat_test(self.data.count, self.data.kind.clone(), ||{
            execute_context_test(test.clone())
        });
    }
}


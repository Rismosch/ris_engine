use std::marker::PhantomData;

use crate::{icontext::IContext, repeat_test::RepeatData};

pub struct RepeatContextTest<TContext: IContext> {
    phantom_data: PhantomData<TContext>,
    data: RepeatData,
}

impl<TContext: IContext + std::panic::RefUnwindSafe + std::panic::UnwindSafe> RepeatContextTest<TContext> {
    pub fn new(data: RepeatData) -> Self {
        RepeatContextTest {phantom_data: PhantomData::default(), data}
    }
    
    pub fn run(&self, test_fn: fn(TContext)) {
        panic!()
    }
}


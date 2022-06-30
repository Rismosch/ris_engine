use crate::{repeat_single_thread_test::RepeatSingleThreadTest, repeat_context_test::RepeatContextTest};

pub struct RepeatTest {}

impl RepeatTest {
    pub fn single_thread() -> RepeatSingleThreadTest {
        panic!()
    }

    pub fn context() -> RepeatContextTest {
        panic!()
    }

    pub fn run(test_fn: fn()){
        panic!()
    }
}
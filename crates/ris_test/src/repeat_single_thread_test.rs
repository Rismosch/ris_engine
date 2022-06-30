use crate::repeat_single_thread_context_test::RepeatSingleThreadContextTest;

pub struct RepeatSingleThreadTest {}

impl RepeatSingleThreadTest {
    pub fn context() -> RepeatSingleThreadContextTest {
        panic!()
    }

    pub fn run(test_fn: fn()){
        panic!()
    }
}


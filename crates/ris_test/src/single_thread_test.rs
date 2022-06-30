use crate::single_thread_context_test::SingleThreadContextTest;

pub struct SingleThreadTest {}

impl SingleThreadTest {
    pub fn context() -> SingleThreadContextTest {
        panic!()
    }

    pub fn run(test_fn: fn()){
        panic!()
    }
}
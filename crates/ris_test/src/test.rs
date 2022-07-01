use crate::{
    context_test::ContextTest,
    icontext::IContext,
    repeat_test::{RepeatKind, RepeatTest},
    single_thread_test::SingleThreadTest,
};

pub struct Test {}

pub fn test() -> Test {
    Test {}
}

impl Test {
    pub fn repeat(&self, repeats: u32) -> RepeatTest {
        RepeatTest::new(repeats, RepeatKind::Repeat)
    }

    pub fn retry(&self, retries: u32) -> RepeatTest {
        RepeatTest::new(retries, RepeatKind::Retry)
    }

    pub fn single_thread(&self) -> SingleThreadTest {
        SingleThreadTest::default()
    }

    pub fn context<TContext: IContext + std::panic::RefUnwindSafe + std::panic::UnwindSafe>(
        &self,
    ) -> ContextTest<TContext> {
        ContextTest::default()
    }

    pub fn run<T: FnOnce()>(&self, test: T) {
        test();
    }
}

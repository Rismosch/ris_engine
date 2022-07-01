use crate::{repeat_test::{RepeatTest, RepeatKind}, single_thread_test::SingleThreadTest, icontext::IContext, context_test::ContextTest};

pub struct Test {}

pub fn test() -> Test {
    Test {  }
}

impl Test {
    pub fn repeat(&self, repeats: u32) -> RepeatTest {
        RepeatTest::new(repeats, RepeatKind::Repeat)
    }

    pub fn retry(&self, retries: u32) -> RepeatTest {
        RepeatTest::new(retries, RepeatKind::Retry)
    }

    pub fn single_thread(&self) -> SingleThreadTest {
        SingleThreadTest::new()
    }

    pub fn context<TContext: IContext + std::panic::RefUnwindSafe + std::panic::UnwindSafe>(&self) -> ContextTest<TContext> {
        ContextTest::new()
    }

    pub fn run<T: FnOnce()>(&self, test: T) {
        test();
    }
}
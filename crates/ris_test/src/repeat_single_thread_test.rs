use crate::{repeat_test::{RepeatKind, execute_repeat_test, RepeatData}, repeat_single_thread_context_test::RepeatSingleThreadContextTest, single_thread_test::execute_single_thread_test};

pub struct RepeatSingleThreadTest {
    data: RepeatData,
}

impl RepeatSingleThreadTest {
    pub fn new(data: RepeatData) -> Self {
        RepeatSingleThreadTest { data }
    }

    pub fn context() -> RepeatSingleThreadContextTest {
        panic!()
    }

    pub fn run(&self, test: fn()){
        execute_single_thread_test(||
            execute_repeat_test(self.data.count, self.data.kind.clone(), test)
        )
    }
}


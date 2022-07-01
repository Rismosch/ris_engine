use crate::{
    icontext::IContext,
    repeat_single_thread_context_test::RepeatSingleThreadContextTest,
    repeat_test::{execute_repeat_test, RepeatData},
    single_thread_test::execute_single_thread_test,
};

pub struct RepeatSingleThreadTest {
    data: RepeatData,
}

impl RepeatSingleThreadTest {
    pub fn new(data: RepeatData) -> Self {
        RepeatSingleThreadTest { data }
    }

    pub fn context<TContext: IContext + std::panic::RefUnwindSafe + std::panic::UnwindSafe>(
        &self,
    ) -> RepeatSingleThreadContextTest<TContext> {
        RepeatSingleThreadContextTest::new(self.data.clone())
    }

    pub fn run<TFnMut: FnMut() + Clone + std::panic::UnwindSafe + std::panic::RefUnwindSafe>(
        &self,
        test: TFnMut,
    ) {
        execute_repeat_test(self.data.count, self.data.kind.clone(), || {
            execute_single_thread_test(test.clone())
        });

        // execute_single_thread_test(||
        //     execute_repeat_test(self.data.count, self.data.kind.clone(), test)
        // )
    }
}

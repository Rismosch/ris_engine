use std::sync::atomic::{AtomicBool, Ordering};

use crate::{icontext::IContext, single_thread_context_test::SingleThreadContextTest};

#[derive(Default)]
pub struct SingleThreadTest {}

impl SingleThreadTest {
    pub fn context<TContext: IContext + std::panic::RefUnwindSafe + std::panic::UnwindSafe>(
        &self,
    ) -> SingleThreadContextTest<TContext> {
        SingleThreadContextTest::default()
    }

    pub fn run<TFnMut: FnMut() + std::panic::UnwindSafe>(&self, test: TFnMut) {
        execute_single_thread_test(test);
    }
}

static mut THREAD_BLOCKED: AtomicBool = AtomicBool::new(false);
pub fn execute_single_thread_test<TFnMut: FnMut() + std::panic::UnwindSafe>(test: TFnMut) {
    loop {
        let result = unsafe {
            THREAD_BLOCKED.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
        };

        if result.is_err() {
            std::thread::yield_now();
            continue;
        }

        let result = std::panic::catch_unwind(test);

        let _ = unsafe { THREAD_BLOCKED.swap(false, Ordering::SeqCst) };

        assert!(result.is_ok());

        break;
    }
}

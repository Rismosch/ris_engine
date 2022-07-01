use std::sync::atomic::{AtomicBool, Ordering};

use crate::{single_thread_context_test::SingleThreadContextTest, icontext::IContext};

pub struct SingleThreadTest {}

static mut THREAD_BLOCKED: AtomicBool = AtomicBool::new(false);

impl SingleThreadTest {
    pub fn new() -> Self {
        SingleThreadTest {  }
    }

    pub fn context<TContext: IContext + std::panic::RefUnwindSafe + std::panic::UnwindSafe>(&self) -> SingleThreadContextTest<TContext> {
        SingleThreadContextTest::new()
    }

    pub fn run<TFnMut: FnMut() + std::panic::UnwindSafe>(&self, test_fn:TFnMut){
        execute_single_thread_test(test_fn);
    }
}

pub fn execute_single_thread_test<TFnMut: FnMut() + std::panic::UnwindSafe>(test: TFnMut){
    loop {
        let result = unsafe {
            THREAD_BLOCKED.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
        };

        if result.is_err() {
            std::thread::yield_now();
            continue;
        }

        let result = std::panic::catch_unwind(test);

        let _ = unsafe { THREAD_BLOCKED.swap(false, Ordering::Relaxed) };

        assert!(result.is_ok());

        break;
    }
}
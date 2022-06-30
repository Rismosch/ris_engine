use std::sync::atomic::{AtomicBool, Ordering};

use crate::{single_thread_context_test::SingleThreadContextTest, icontext::IContext};

pub struct SingleThreadTest {}

static mut THREAD_BLOCKED: AtomicBool = AtomicBool::new(false);

impl SingleThreadTest {
    pub fn new() -> Self {
        SingleThreadTest {  }
    }

    pub fn context<TContext: IContext + std::panic::RefUnwindSafe + std::panic::UnwindSafe>() -> SingleThreadContextTest<TContext> {
        SingleThreadContextTest::new()
    }

    pub fn run(&self, test_fn: fn()){
        execute_single_thread_test(test_fn);
    }
}

pub fn execute_single_thread_test<T: FnMut() + std::panic::UnwindSafe>(test: T){
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
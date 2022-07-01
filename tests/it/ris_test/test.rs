use ris_test::{icontext::IContext, test::ris_test};

static mut TEST_CALLS: i32 = 0;
#[test]
fn should_build_test() {
    unsafe {
        TEST_CALLS = 0;
    }

    ris_test().run(|| unsafe { TEST_CALLS += 1 });

    let calls = unsafe { TEST_CALLS };

    assert_eq!(calls, 1);
}

static mut REPEAT_CALLS: i32 = 0;
#[test]
fn should_build_repeat_test() {
    unsafe {
        REPEAT_CALLS = 0;
    }

    ris_test().repeat(10).run(|| unsafe { REPEAT_CALLS += 1 });

    let calls = unsafe { REPEAT_CALLS };

    assert_eq!(calls, 10);
}

static mut RETRY_CALLS: i32 = 0;
#[test]
fn should_build_retry_test() {
    unsafe {
        RETRY_CALLS = 0;
    }

    ris_test().retry(10).run(|| unsafe { RETRY_CALLS += 1 });

    let calls = unsafe { RETRY_CALLS };

    assert_eq!(calls, 1);
}

static mut SINGLE_THREAD_CALLS: i32 = 0;
#[test]
fn should_build_single_thread_test() {
    unsafe {
        SINGLE_THREAD_CALLS = 0;
    }

    ris_test()
        .single_thread()
        .run(|| unsafe { SINGLE_THREAD_CALLS += 1 });

    let calls = unsafe { SINGLE_THREAD_CALLS };

    assert_eq!(calls, 1);
}

struct Context {}

impl IContext for Context {
    fn setup() -> Self {
        Context {}
    }

    fn teardown(&mut self) {}
}

static mut CONTEXT_CALLS: i32 = 0;
#[test]
fn should_build_context_test() {
    unsafe {
        CONTEXT_CALLS = 0;
    }

    ris_test()
        .context::<Context>()
        .run(|_| unsafe { CONTEXT_CALLS += 1 });

    let calls = unsafe { CONTEXT_CALLS };

    assert_eq!(calls, 1);
}

static mut REPEAT_SINGLE_THREAD_CALLS: i32 = 0;
#[test]
fn should_build_repeat_single_thread_test() {
    unsafe {
        REPEAT_SINGLE_THREAD_CALLS = 0;
    }

    ris_test()
        .repeat(10)
        .single_thread()
        .run(|| unsafe { REPEAT_SINGLE_THREAD_CALLS += 1 });

    let calls = unsafe { REPEAT_SINGLE_THREAD_CALLS };

    assert_eq!(calls, 10);
}

struct RepeatContext {}
impl IContext for RepeatContext {
    fn setup() -> Self {
        RepeatContext {}
    }

    fn teardown(&mut self) {}
}

static mut REPEAT_CONTEXT_CALLS: i32 = 0;
#[test]
fn should_build_repeat_context_test() {
    unsafe {
        REPEAT_CONTEXT_CALLS = 0;
    }

    ris_test()
        .repeat(10)
        .context::<RepeatContext>()
        .run(|_| unsafe { REPEAT_CONTEXT_CALLS += 1 });

    let calls = unsafe { REPEAT_CONTEXT_CALLS };

    assert_eq!(calls, 10);
}

struct RepeatSingleThreadContext {}
impl IContext for RepeatSingleThreadContext {
    fn setup() -> Self {
        RepeatSingleThreadContext {}
    }

    fn teardown(&mut self) {}
}

static mut REPEAT_SINGLE_THREAD_CONTEXT_CALLS: i32 = 0;
#[test]
fn should_build_repeat_single_thread_context_test() {
    unsafe {
        REPEAT_SINGLE_THREAD_CONTEXT_CALLS = 0;
    }

    ris_test()
        .repeat(10)
        .single_thread()
        .context::<RepeatSingleThreadContext>()
        .run(|_| unsafe { REPEAT_SINGLE_THREAD_CONTEXT_CALLS += 1 });

    let calls = unsafe { REPEAT_SINGLE_THREAD_CONTEXT_CALLS };

    assert_eq!(calls, 10);
}

static mut RETRY_SINGLE_THREAD_CALLS: i32 = 0;
#[test]
fn should_build_retry_single_thread_test() {
    unsafe {
        RETRY_SINGLE_THREAD_CALLS = 0;
    }

    ris_test()
        .retry(10)
        .single_thread()
        .run(|| unsafe { RETRY_SINGLE_THREAD_CALLS += 1 });

    let calls = unsafe { RETRY_SINGLE_THREAD_CALLS };

    assert_eq!(calls, 1);
}

struct RetryContext {}
impl IContext for RetryContext {
    fn setup() -> Self {
        RetryContext {}
    }

    fn teardown(&mut self) {}
}

static mut RETRY_CONTEXT_CALLS: i32 = 0;
#[test]
fn should_build_retry_context_test() {
    unsafe {
        RETRY_CONTEXT_CALLS = 0;
    }

    ris_test()
        .retry(10)
        .context::<RetryContext>()
        .run(|_| unsafe { RETRY_CONTEXT_CALLS += 1 });

    let calls = unsafe { RETRY_CONTEXT_CALLS };

    assert_eq!(calls, 1);
}

struct RetrySingleThreadContext {}
impl IContext for RetrySingleThreadContext {
    fn setup() -> Self {
        RetrySingleThreadContext {}
    }

    fn teardown(&mut self) {}
}

static mut RETRY_SINGLE_THREAD_CONTEXT_CALLS: i32 = 0;
#[test]
fn should_build_retry_single_thread_context_test() {
    unsafe {
        RETRY_SINGLE_THREAD_CONTEXT_CALLS = 0;
    }

    ris_test()
        .repeat(10)
        .single_thread()
        .context::<RetrySingleThreadContext>()
        .run(|_| unsafe { RETRY_SINGLE_THREAD_CONTEXT_CALLS += 1 });

    let calls = unsafe { RETRY_SINGLE_THREAD_CONTEXT_CALLS };

    assert_eq!(calls, 10);
}

struct SingleThreadContext {}
impl IContext for SingleThreadContext {
    fn setup() -> Self {
        SingleThreadContext {}
    }

    fn teardown(&mut self) {}
}

static mut SINGLE_THREAD_CONTEXT_CALLS: i32 = 0;
#[test]
fn should_build_single_thread_context_test() {
    unsafe {
        SINGLE_THREAD_CONTEXT_CALLS = 0;
    }

    ris_test()
        .single_thread()
        .context::<SingleThreadContext>()
        .run(|_| unsafe { SINGLE_THREAD_CONTEXT_CALLS += 1 });

    let calls = unsafe { SINGLE_THREAD_CONTEXT_CALLS };

    assert_eq!(calls, 1);
}

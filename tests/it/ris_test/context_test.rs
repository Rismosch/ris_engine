use ris_test::{test::test, icontext::IContext};

struct ContextSucceed {
    number: i32,
}

impl IContext for ContextSucceed {
    fn setup() -> Self {
        unsafe {
            SUCCEED_SETUP_CALLED = true;
        }
        ContextSucceed { number: -13}
    }

    fn teardown(&mut self) {
        unsafe {
            SUCCEED_TEARDOWN_NUMBER = self.number;
        }
    }
}

static mut SUCCEED_SETUP_CALLED: bool = false;
static mut SUCCEED_TEARDOWN_NUMBER: i32 = 0;
#[test]
fn should_succeed() {
    unsafe {
        SUCCEED_SETUP_CALLED = false;
        SUCCEED_TEARDOWN_NUMBER = 0;
    }

    test()
    .context::<ContextSucceed>()
    .run(move |context| {
        assert_eq!(context.number, -13);
        context.number = 42;
    });

    let setup_called = unsafe {
        SUCCEED_SETUP_CALLED
    };

    let teardown_number = unsafe {
        SUCCEED_TEARDOWN_NUMBER
    };

    assert!(setup_called);
    assert_eq!(teardown_number, 42);
}

struct ContextFail {
    number: i32,
}

impl IContext for ContextFail {
    fn setup() -> Self {
        unsafe {
            FAIL_SETUP_CALLED = true;
        }
        ContextFail { number: -13}
    }

    fn teardown(&mut self) {
        unsafe {
            FAIL_TEARDOWN_NUMBER = self.number;
        }
    }
}

static mut FAIL_SETUP_CALLED: bool = false;
static mut FAIL_TEARDOWN_NUMBER: i32 = 0;
#[test]
fn should_fail() {
    unsafe {
        FAIL_SETUP_CALLED = false;
        FAIL_TEARDOWN_NUMBER = 0;
    }

    let result = std::panic::catch_unwind(|| {
        test()
        .context::<ContextFail>()
        .run(move |context| {
            assert_eq!(context.number, -13);
            context.number = 42;
            panic!()
        });
    });

    assert!(result.is_err());

    let setup_called = unsafe {
        FAIL_SETUP_CALLED
    };

    let teardown_number = unsafe {
        FAIL_TEARDOWN_NUMBER
    };

    assert!(setup_called);
    assert_eq!(teardown_number, 42);
}
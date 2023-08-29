use ris_util::ris_error::RisError;

#[derive(Debug)]
struct TestError;

impl std::fmt::Display for TestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "i am a test error :)")
    }
}

impl std::error::Error for TestError {}

fn inner_result_fn(value: i32, ok: bool) -> Result<i32, RisError> {
    ris_util::unroll!(
        inner_inner_result_fn(value, ok),
        "inner result {} {}",
        value,
        ok
    )
}

fn inner_inner_result_fn(value: i32, ok: bool) -> Result<i32, TestError> {
    if ok {
        Ok(value)
    } else {
        Err(TestError)
    }
}

fn inner_option_fn(value: i32, ok: bool) -> Result<i32, RisError> {
    ris_util::unroll_option!(
        inner_inner_option_fn(value, ok),
        "inner option {} {}",
        value,
        ok
    )
}

fn inner_inner_option_fn(value: i32, ok: bool) -> Option<i32> {
    if ok {
        Some(value)
    } else {
        None
    }
}

#[test]
fn should_unroll_result() {
    let result1 = ris_util::unroll!(inner_result_fn(42, true), "result");
    let result2 = ris_util::unroll!(inner_result_fn(42, false), "result");

    assert_eq!(result1.unwrap(), 42);
    assert_eq!(format!("{}", result2.unwrap_err()), "i am a test error :)\n        \"inner result 42 false\", tests\\it\\ris_util\\ris_error.rs:15\n        \"result\", tests\\it\\ris_util\\ris_error.rs:51");
}

#[test]
fn should_unroll_option() {
    let result1 = ris_util::unroll!(inner_option_fn(42, true), "option");
    let result2 = ris_util::unroll!(inner_option_fn(42, false), "option");

    assert_eq!(result1.unwrap(), 42);
    assert_eq!(format!("{}", result2.unwrap_err()), "Option was None\n        \"inner option 42 false\", tests\\it\\ris_util\\ris_error.rs:32\n        \"option\", tests\\it\\ris_util\\ris_error.rs:60");
}

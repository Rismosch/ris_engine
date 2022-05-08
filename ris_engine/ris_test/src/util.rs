pub fn retry(count: usize, test: fn() -> ()) {
    for _ in 0..count - 1 {
        let result = std::panic::catch_unwind(test);

        if result.is_ok() {
            return;
        }
    }

    test();
}

pub fn repeat(count: usize, test: fn() -> ()) {
    for _ in 0..count {
        test();
    }
}

pub fn wrap<T>(teardown: fn() -> (), test: T)
where
    T: FnOnce() + std::panic::UnwindSafe,
{
    let result = std::panic::catch_unwind(test);
    teardown();
    assert!(result.is_ok());
}

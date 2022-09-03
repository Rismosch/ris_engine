pub fn repeat<F: FnMut() + Clone>(repeats: u32, test: F) {
    for _ in 0..repeats {
        test.clone()();
    }
}

pub fn retry<F: FnMut() + Clone + std::panic::UnwindSafe>(retries: u32, test: F) {
    for _ in 0..retries - 1 {
        let result = std::panic::catch_unwind(test.clone());

        if result.is_ok() {
            return;
        }
    }

    let result = std::panic::catch_unwind(test);
    assert!(result.is_ok(), "failed {} times", retries);
}
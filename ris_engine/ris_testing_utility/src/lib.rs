pub fn retry(retries: usize, test: fn() -> ()) {
    for _ in 0..retries - 1 {
        let result = std::panic::catch_unwind(test);

        if result.is_ok() {
            return;
        }
    }

    test();
}

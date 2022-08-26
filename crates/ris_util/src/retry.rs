pub fn retry<F: FnMut() + Clone + std::panic::UnwindSafe>(retries: u32, test: F) {
    for i in 0..retries {
        let result = std::panic::catch_unwind(test.clone());

        if i == retries - 1 {
            assert!(result.is_ok(), "failed {} times", i);
        } else if result.is_ok() {
            return ;
        }
    }
}
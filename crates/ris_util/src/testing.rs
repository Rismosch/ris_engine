pub fn repeat<F: FnMut() + Clone>(repeats: usize, test: F) {
    for _i in 0..repeats {
        test.clone()();
    }
}

pub fn retry<F: FnMut() + Clone + std::panic::UnwindSafe>(retries: usize, test: F) {
    for _ in 0..retries - 1 {
        let result = std::panic::catch_unwind(test.clone());

        if result.is_ok() {
            return;
        }
    }

    let result = std::panic::catch_unwind(test);
    assert!(result.is_ok(), "failed {} times", retries);
}

pub fn assert_feq(left: f32, right: f32, tolerance: f32) {
    let diff = ris_math::diff(left, right);
    assert!(
        diff < tolerance,
        "expected {} and {} to be within {}, but differed by {}",
        left,
        right,
        tolerance,
        diff
    );
}

/// generates and cleans a temporary directory for tests
#[macro_export]
macro_rules! prep_test_dir {
    () => {{
        let mut test_name = String::from(ris_util::function!());
        let test_path = test_name.replace("::", "/");
        let sanitized_test_path = ris_util::path::sanitize(&test_path, false);

        let executable_string = std::env::args().next().expect("no cli args");
        let executable_path = std::path::PathBuf::from(executable_string);
        let executable_directory = executable_path.parent().expect("executable has no parent");

        let mut result = std::path::PathBuf::new();
        result.push(executable_directory);
        result.push(sanitized_test_path);

        if result.exists() {
            let _ = std::fs::remove_dir_all(&result);
        }

        let _ = std::fs::create_dir_all(&result);
        if !result.exists() {
            panic!("failed to create \"{:?}\"", &result);
        }

        result
    }};
}

/// this avoids clippy from beeing too smart. clippy flags some clones as redundant, which defeats
/// the purpose of some tests, especially when multiple copies are passed to other threads.
pub fn duplicate<T: Clone>(value: &T) -> T {
    value.clone()
}

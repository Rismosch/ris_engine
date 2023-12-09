pub fn repeat<F: FnMut(usize) + Clone>(repeats: usize, test: F) {
    for i in 0..repeats {
        test.clone()(i);
    }
}

pub fn retry<F: FnMut() + Clone + std::panic::UnwindSafe>(retries: usize, test: F) {
    for _i in 0..retries - 1 {
        let result = std::panic::catch_unwind(test.clone());

        if result.is_ok() {
            return;
        }
    }

    let result = std::panic::catch_unwind(test);
    assert!(result.is_ok(), "failed {} times", retries);
}

#[macro_export]
macro_rules! assert_feq {
    ($left:expr, $right:expr, $tolerance:expr) => {{
        $crate::assert_feq!($left, $right, $tolerance, "");
    }};
    ($left:expr, $right:expr, $tolerance:expr, $($arg:tt)*) => {{
        let diff = ris_math::diff($left, $right);
        let message = format!($($arg)*);
        assert!(
            diff < $tolerance,
            "expected {} and {} to be within {}, but differed by {}. {}",
            $left,
            $right,
            $tolerance,
            diff,
            message
        );
    }};
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

#[cfg(not(miri))]
pub fn miri_choose<T>(value: T, _: T) -> T {
    value
}

#[cfg(miri)]
pub fn miri_choose<T>(_: T, value: T) -> T {
    value
}

pub fn repeat<F: FnMut() + Clone>(repeats: u32, test: F) {
    for _i in 0..repeats {
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
    ($test_name:expr) => {
        {
            let executable_string = std::env::args().next().expect("no cli args");
            let executable_path = std::path::PathBuf::from(executable_string);
            let working_directory = executable_path.parent().expect("executable has no parent");

            let test_file = std::path::PathBuf::from(file!());
            let test_dir = test_file.parent().expect("test has no parent");
            let test_dir_path = std::path::PathBuf::from(test_dir);
            let test_dir_name = test_dir_path.file_name().expect("test dir has no name");
            let test_file_name = test_file.file_name().expect("test has no name");

            let mut result = std::path::PathBuf::new();
            result.push(working_directory);
            result.push("ris_util_test_dir");
            result.push(test_dir_name);
            result.push(test_file_name);
            result.push($test_name);

            if result.exists() {
                let _ = std::fs::remove_dir_all(&result);
            }

            let _ = std::fs::create_dir_all(&result);
            if !result.exists() {
                panic!("failed to create \"{:?}\"", &result);
            }

            result
        }
    }
}

pub fn repeat<F: FnMut(usize) + Clone>(repeats: usize, test: F) {
    for i in 0..repeats {
        test.clone()(i);
        //let mut clone = test.clone();
        //let result = std::panic::catch_unwind(move||(clone)(i));

        //if result.is_err() {
        //    panic!("failed at iteration {}", i);
        //}
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

pub const MIN_NORM: f32 = 0.000_001f32;

#[macro_export]
macro_rules! assert_feq {
    ($left:expr, $right:expr) => {{
        $crate::assert_feq!($left, $right, $crate::testing::MIN_NORM, "");
    }};
    ($left:expr, $right:expr, $tolerance:expr) => {{
        $crate::assert_feq!($left, $right, $tolerance, "");
    }};
    ($left:expr, $right:expr, $tolerance:expr, $($arg:tt)*) => {{
        let diff = f32::abs($left - $right);
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

#[derive(PartialEq, Eq)]
pub enum BytesEqualResult {
    Equal,
    DifferentLengths,
    DifferendAt(usize),
}

pub fn bytes_eq(left: &[u8], right: &[u8]) -> bool {
    bytes_eq_detailed(left, right) == BytesEqualResult::Equal
}

pub fn bytes_eq_detailed(left: &[u8], right: &[u8]) -> BytesEqualResult {
    if left.len() != right.len() {
        return BytesEqualResult::DifferentLengths;
    }

    for i in 0..left.len() {
        let left = left[i];
        let right = right[i];

        if left != right {
            return BytesEqualResult::DifferendAt(i);
        }
    }

    BytesEqualResult::Equal
}

#[macro_export]
macro_rules! assert_bytes_eq {
    ($left:expr, $right:expr) => {{
        $crate::assert_bytes_eq!($left, $right, "");
    }};
    ($left:expr, $right:expr, $($arg:tt)*) => {{
        use ris_util::testing::BytesEqualResult;

        let result = ris_util::testing::bytes_eq_detailed(&$left, &$right);
        match result {
            BytesEqualResult::Equal => (),
            BytesEqualResult::DifferentLengths => panic!(
                "left and right have different lengths: {} != {} {}",
                $left.len(),
                $right.len(),
                format!($($arg)*),
            ),
            BytesEqualResult::DifferendAt(i) => {
                let min = i.saturating_sub(5);
                let max = usize::min(i + 6, $left.len());

                let mut left_string = String::from("[");
                let mut diff_string = String::from(" ");
                let mut right_string = String::from("[");

                if min != 0 {
                    left_string.push_str("...");
                    diff_string.push_str("   ");
                    right_string.push_str("...");
                }

                for j in min..max {
                    let left_value = format!(", 0x{:02X}", $left[j]);
                    let right_value = format!(", 0x{:02X}", $right[j]);

                    let diff_value = if i == j {
                        "  xxxx"
                    } else {
                        "      "
                    };

                    left_string.push_str(&left_value);
                    diff_string.push_str(diff_value);
                    right_string.push_str(&right_value);
                }

                if max != $left.len() {
                    left_string.push_str(", ...");
                    diff_string.push_str("     ");
                    right_string.push_str(", ...");
                }

                left_string.push_str("]");
                diff_string.push_str(" ");
                right_string.push_str("]");

                panic!(
                    "left and right differed at {}:\n{}\n{}\n{}\n{}",
                    i,
                    left_string,
                    diff_string,
                    right_string,
                    format!($($arg)*),
                )
            },
        }
    }};
}

#[macro_export]
macro_rules! assert_vec2_eq {
    ($left:expr, $right:expr) => {{
        $crate::assert_feq!($left.0, $right.0);
        $crate::assert_feq!($left.1, $right.1);
    }};
}

#[macro_export]
macro_rules! assert_vec3_eq {
    ($left:expr, $right:expr) => {{
        $crate::assert_vec3_eq!($left, $right, $crate::testing::MIN_NORM);
    }};
    ($left:expr, $right:expr, $tolerance:expr) => {{
        $crate::assert_feq!($left.0, $right.0, $tolerance);
        $crate::assert_feq!($left.1, $right.1, $tolerance);
        $crate::assert_feq!($left.2, $right.2, $tolerance);
    }};
}

#[macro_export]
macro_rules! assert_vec4_eq {
    ($left:expr, $right:expr) => {{
        $crate::assert_feq!($left.0, $right.0);
        $crate::assert_feq!($left.1, $right.1);
        $crate::assert_feq!($left.2, $right.2);
        $crate::assert_feq!($left.3, $right.3);
    }};
}

#[macro_export]
macro_rules! assert_quat_eq {
    ($left:expr, $right:expr) => {{
        $crate::assert_quat_eq!($left, $right, $crate::testing::MIN_NORM);
    }};
    ($left:expr, $right:expr, $tolerance:expr) => {{
        let left = ris_math::vector::Vec4::from($left);
        let right = ris_math::vector::Vec4::from($right);
        let min_norm = ris_math::vector::Vec4::init($tolerance);

        // a quaternion with negated components represents the same rotation.
        // check the diff twice: once normal and once with negated components.
        let abs1 = (left - right).abs();
        let abs2 = (left + right).abs();
        let result1 = abs1.less_than(min_norm).all();
        let result2 = abs2.less_than(min_norm).all();

        // when both checks fail, the quaternions are not equal
        if !result1 && !result2 {
            let failed_abs = if !result1 {
                abs1
            } else {
                abs2
            };

            panic!("expected {:?} and {:?} to be within {:?} but differed by {:?}", $left, $right, $tolerance, failed_abs);
        }
    }};
}

/// generates and cleans a temporary directory for tests
#[macro_export]
macro_rules! prep_test_dir {
    () => {{
        let mut test_name = String::from(ris_util::function!());
        let test_path = test_name.replace("::", "/");
        let sanitized_test_path = ris_file::path::sanitize(&test_path, false);

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
/// the purpose of some tests, especially when the copy implementation is being tested or multiple
/// copies are passed to other threads.
pub fn duplicate<T: Clone>(value: &T) -> T {
    value.clone()
}

#[cfg(not(miri))]
pub fn miri_choose<T>(not_miri: T, _miri: T) -> T {
    not_miri
}

#[cfg(miri)]
pub fn miri_choose<T>(_not_miri: T, miri: T) -> T {
    miri
}

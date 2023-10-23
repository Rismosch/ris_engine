use std::path::Path;
use std::path::PathBuf;

use ris_util::fallback_file::FallbackFileAppend;

#[test]
fn should_create_directories() {
    let mut test_dir = ris_util::prep_test_dir!();
    test_dir.push("my");
    test_dir.push("awesome");
    test_dir.push("fallback");
    test_dir.push("directory");
    let _appender = FallbackFileAppend::new(&test_dir, ".test").unwrap();

    let mut current_file = PathBuf::from(&test_dir);
    current_file.push("current.test");

    let mut old_directory = PathBuf::from(&test_dir);
    old_directory.push("old");

    assert!(test_dir.exists());
    assert!(current_file.exists());
    assert!(old_directory.exists());

    assert!(test_dir.metadata().unwrap().is_dir());
    assert!(current_file.metadata().unwrap().is_file());
    assert!(old_directory.metadata().unwrap().is_dir());
}

#[test]
fn should_delete_expired_files() {
    panic!()
}

#[test]
fn should_move_current_file() {
    panic!()
}

#[test]
fn should_create_current_file_with_timestamp() {
    panic!()
}


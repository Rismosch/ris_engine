use std::io::Read;
use std::io::Write;
use std::io::Seek;
use std::path::PathBuf;

use chrono::DateTime;
use chrono::Duration;
use chrono::Local;

use ris_util::fallback_file::FallbackFileAppend;

#[test]
fn should_create_directories() {
    let mut test_dir = ris_util::prep_test_dir!();
    test_dir.push("my");
    test_dir.push("awesome");
    test_dir.push("fallback");
    test_dir.push("directory");
    FallbackFileAppend::new(&test_dir, ".test").unwrap();

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
    let test_dir = ris_util::prep_test_dir!();

    let mut old_dir = PathBuf::from(&test_dir);
    old_dir.push("old");
    std::fs::create_dir_all(&old_dir).unwrap();

    let mut file_paths = Vec::new();
    for i in 0..ris_util::fallback_file::OLD_FILE_COUNT {
        let mut file_path = PathBuf::from(&old_dir);
        file_path.push(format!("{}", i));

        std::fs::File::create(&file_path).unwrap();
        file_paths.push(file_path);
    }

    FallbackFileAppend::new(&test_dir, ".test").unwrap();

    assert!(!file_paths[0].exists()); // first one expired and got deleted
    assert!(file_paths[1].exists());
    assert!(file_paths[2].exists());
    assert!(file_paths[3].exists());
    assert!(file_paths[4].exists());
    assert!(file_paths[5].exists());
    assert!(file_paths[6].exists());
    assert!(file_paths[7].exists());
    assert!(file_paths[8].exists());
    assert!(file_paths[9].exists());
}

#[test]
fn should_create_current_file_with_timestamp() {
    let test_dir = ris_util::prep_test_dir!();
    FallbackFileAppend::new(&test_dir, ".test").unwrap();

    let mut current_file_path = PathBuf::from(test_dir);
    current_file_path.push("current.test");

    let mut file = std::fs::File::open(current_file_path).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    let first_line = content.lines().next().unwrap();
    let file_date = DateTime::parse_from_rfc2822(first_line).unwrap().with_timezone(&Local);
    let now = Local::now();

    let diff = now - file_date;
    let one_second = Duration::seconds(1);
    assert!(diff < one_second);
}

#[test]
fn should_move_current_file() {
    let test_dir = ris_util::prep_test_dir!();
    
    //for _ in 0..100 {
    //    std::thread::sleep(std::time::Duration::from_secs(1));
    //    FallbackFileAppend::new(&test_dir, ".test").unwrap();
    //}

    panic!();
}

#[test]
fn should_append_to_current_file() {
    panic!();
}



use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::Write;
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
    FallbackFileAppend::new(&test_dir, ".test", 10).unwrap();

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
    for i in 0..10 {
        let mut file_path = PathBuf::from(&old_dir);
        file_path.push(format!("{}", i));

        std::fs::File::create(&file_path).unwrap();
        file_paths.push(file_path);
    }

    FallbackFileAppend::new(&test_dir, ".test", 10).unwrap();

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
    FallbackFileAppend::new(&test_dir, ".test", 10).unwrap();

    let mut current_file_path = PathBuf::from(&test_dir);
    current_file_path.push("current.test");

    let mut file = std::fs::File::open(current_file_path).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    let first_line = content.lines().next().unwrap();
    let file_date = DateTime::parse_from_rfc3339(first_line)
        .unwrap()
        .with_timezone(&Local);
    let now = Local::now();

    let diff = now - file_date;
    let one_second = Duration::seconds(1);
    assert!(diff < one_second);
}

#[test]
fn should_move_current_file() {
    let test_dir = ris_util::prep_test_dir!();
    FallbackFileAppend::new(&test_dir, ".test", 10).unwrap();

    let mut current_path = PathBuf::from(&test_dir);
    current_path.push("current.test");
    let mut old_path = PathBuf::from(&test_dir);
    old_path.push("old");

    // move file 1
    // should use first line as file name
    std::fs::remove_file(&current_path).unwrap();
    let mut current_file = std::fs::File::create(&current_path).unwrap();
    writeln!(current_file, "i am a unique file").unwrap();
    FallbackFileAppend::new(&test_dir, ".test", 10).unwrap();
    let mut file_path = PathBuf::from(&old_path);
    file_path.push("i am a unique file.test");
    let mut file = std::fs::File::open(&file_path).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    assert!(file_path.exists());
    assert_eq!(content, "i am a unique file\n");

    // move file 2
    // should use first line as file name, sanitizing invalid chars
    std::fs::remove_file(&current_path).unwrap();
    let mut current_file = std::fs::File::create(&current_path).unwrap();
    writeln!(current_file, "i am not unique :(").unwrap();
    FallbackFileAppend::new(&test_dir, ".test", 10).unwrap();
    let mut file_path = PathBuf::from(&old_path);
    file_path.push("i am not unique _(.test");
    let mut file = std::fs::File::open(&file_path).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    assert!(file_path.exists());
    assert_eq!(content, "i am not unique :(\n");

    // move file 3
    // should generate new unique filename, which does not correspont to its first line
    std::fs::remove_file(&current_path).unwrap();
    let mut current_file = std::fs::File::create(&current_path).unwrap();
    writeln!(current_file, "i am not unique :(").unwrap();
    FallbackFileAppend::new(&test_dir, ".test", 10).unwrap();
    let mut entries = std::fs::read_dir(&old_path).unwrap();
    let entry = entries.next().unwrap().unwrap();
    let unique_path = entry.path();
    let mut file = std::fs::File::open(&unique_path).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    assert_ne!(unique_path, file_path);
    assert!(unique_path.exists());
    assert_eq!(content, "i am not unique :(\n");
}

#[test]
fn should_give_access_to_current_file() {
    let test_dir = ris_util::prep_test_dir!();
    let mut appender = FallbackFileAppend::new(&test_dir, ".test", 10).unwrap();

    let current_file = appender.current();
    let message = "i am a very important message\n";
    let bytes = message.as_bytes();
    current_file.seek(SeekFrom::End(0)).unwrap();
    let _ = current_file.write(bytes).unwrap();

    drop(appender);

    let mut current_path = PathBuf::from(&test_dir);
    current_path.push("current.test");
    let mut current_file = std::fs::File::open(current_path).unwrap();
    let mut content = String::new();
    current_file.read_to_string(&mut content).unwrap();

    let lines = content.lines().collect::<Vec<&str>>();
    assert_eq!(lines.len(), 3);
    assert!(DateTime::parse_from_rfc3339(lines[0]).is_ok());
    assert_eq!(lines[1], "");
    assert_eq!(lines[2], "i am a very important message");
}

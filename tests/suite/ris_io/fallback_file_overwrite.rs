use std::io::Read;
use std::io::Write;
use std::path::PathBuf;

use ris_io::fallback_file::FallbackFileOverwrite;

#[test]
fn should_not_create_directories_when_nothing_is_written() {
    let mut test_dir = ris_util::prep_test_dir!();
    test_dir.push("test");

    FallbackFileOverwrite::new(&test_dir, ".test", 10);

    assert!(!test_dir.exists());
}

#[test]
fn should_create_directories() {
    let mut test_dir = ris_util::prep_test_dir!();
    test_dir.push("my");
    test_dir.push("awesome");
    test_dir.push("directory");
    let mut old_dir = PathBuf::from(&test_dir);
    old_dir.push("old");

    let overwriter = FallbackFileOverwrite::new(&test_dir, ".test", 10);

    let content = "hello world".as_bytes();
    overwriter.overwrite_current(content).unwrap();

    assert!(test_dir.exists());
    assert!(old_dir.exists());
}

#[test]
fn should_write_file() {
    let test_dir = ris_util::prep_test_dir!();
    let overwriter = FallbackFileOverwrite::new(&test_dir, ".test", 10);

    let content = "hello world".as_bytes();
    overwriter.overwrite_current(content).unwrap();

    let mut current_path = PathBuf::from(&test_dir);
    current_path.push("current.test");

    let mut file = std::fs::File::open(&current_path).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    let lines = content.lines().collect::<Vec<&str>>();

    assert_eq!(lines.len(), 3);
    assert_eq!(lines[0], "0");
    assert_eq!(lines[1], "");
    assert_eq!(lines[2], "hello world");
}

#[test]
fn should_move_file() {
    let test_dir = ris_util::prep_test_dir!();
    let overwriter = FallbackFileOverwrite::new(&test_dir, ".test", 10);

    let mut current_path = PathBuf::from(&test_dir);
    current_path.push("current.test");
    let mut old_path = PathBuf::from(&test_dir);
    old_path.push("old");

    overwriter.overwrite_current("zero".as_bytes()).unwrap();

    // move file 1
    // should use Counter::MAX when first line is not an unsigned integer
    std::fs::remove_file(&current_path).unwrap();
    let mut current_file = std::fs::File::create(&current_path).unwrap();
    writeln!(current_file, "i am incorrectly formatted").unwrap();
    overwriter.overwrite_current("un".as_bytes()).unwrap();
    let mut file_path = PathBuf::from(&old_path);
    file_path.push("4294967295.test");
    let mut file = std::fs::File::open(&file_path).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    assert!(file_path.exists());
    assert_eq!(content, "i am incorrectly formatted\n");

    // move file 2
    // should create a unique filename, when for some reason (ie user modifying the files) creates
    // duplicated filenames
    for i in 1..5 {
        std::fs::remove_file(&current_path).unwrap();
        let mut current_file = std::fs::File::create(&current_path).unwrap();
        writeln!(current_file, "i am incorrectly formatted").unwrap();
        overwriter.overwrite_current("i am correctly formatted".as_bytes()).unwrap();
        let mut file_path = PathBuf::from(&old_path);
        file_path.push(format!("4294967295({}).test", i));
        let mut file = std::fs::File::open(&file_path).unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        assert!(file_path.exists());
        assert_eq!(content, "i am incorrectly formatted\n");
    }

    // move file 3
    // file is correctly formatted and uses the line as its filename
    for i in 0..5 {
        overwriter.overwrite_current("i am correctly formatted".as_bytes()).unwrap();
        let mut file_path = PathBuf::from(&old_path);
        file_path.push(format!("{}.test", i));
        let mut file = std::fs::File::open(&file_path).unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        assert!(file_path.exists());
        assert_eq!(content, format!("{}\n\ni am correctly formatted", i));
    }
}

#[test]
fn should_delete_expired_files() {
    let test_dir = ris_util::prep_test_dir!();

    let mut old_dir = PathBuf::from(&test_dir);
    old_dir.push("old");
    std::fs::create_dir_all(&old_dir).unwrap();

    let mut file_paths = Vec::new();
    for i in 0..5 {
        let mut file_path = PathBuf::from(&old_dir);
        file_path.push(format!("{}", i));

        let mut file = std::fs::File::create(&file_path).unwrap();
        let content = format!("{}\n\nhello world", i);
        ris_io::write(&mut file, content.as_bytes()).unwrap();

        file_paths.push(file_path);
    }

    let overwriter = FallbackFileOverwrite::new(&test_dir, ".test", 5);
    overwriter.overwrite_current("i am new".as_bytes()).unwrap();

    assert!(!file_paths[0].exists()); // first one expired and got deleted
    assert!(file_paths[1].exists());
    assert!(file_paths[2].exists());
    assert!(file_paths[3].exists());
    assert!(file_paths[4].exists());
}

#[test]
fn should_get_available_files() {
    let test_dir = ris_util::prep_test_dir!();
    let overwriter = FallbackFileOverwrite::new(&test_dir, ".test", 10);

    let available_paths = overwriter.available_paths();
    assert_eq!(available_paths.len(), 0);

    let mut old_dir = PathBuf::from(&test_dir);
    old_dir.push("old");
    std::fs::create_dir_all(&old_dir).unwrap();

    let mut file_paths = Vec::new();
    for i in (0..5).rev() {
        let mut file_path = PathBuf::from(&old_dir);
        file_path.push(format!("{}", i));

        std::fs::File::create(&file_path).unwrap();
        file_paths.push(file_path);
    }

    let available_paths = overwriter.available_paths();
    assert_eq!(available_paths.len(), 5);
    assert_eq!(available_paths[0], file_paths[4]);
    assert_eq!(available_paths[1], file_paths[3]);
    assert_eq!(available_paths[2], file_paths[2]);
    assert_eq!(available_paths[3], file_paths[1]);
    assert_eq!(available_paths[4], file_paths[0]);

    overwriter.overwrite_current("i am new".as_bytes()).unwrap();

    let available_paths = overwriter.available_paths();
    assert_eq!(available_paths.len(), 6);
    let mut current_path = PathBuf::from(&test_dir);
    current_path.push("current.test");
    assert_eq!(available_paths[0], current_path);
    assert_eq!(available_paths[1], file_paths[4]);
    assert_eq!(available_paths[2], file_paths[3]);
    assert_eq!(available_paths[3], file_paths[2]);
    assert_eq!(available_paths[4], file_paths[1]);
    assert_eq!(available_paths[5], file_paths[0]);
}

#[test]
fn should_get_file_contents_by_path() {
    let test_dir = ris_util::prep_test_dir!();
    let overwriter = FallbackFileOverwrite::new(&test_dir, ".test", 10);

    assert!(overwriter.available_paths().is_empty());

    overwriter.overwrite_current("un".as_bytes()).unwrap();
    overwriter.overwrite_current("deux".as_bytes()).unwrap();
    overwriter.overwrite_current("trois".as_bytes()).unwrap();
    overwriter.overwrite_current("quatre".as_bytes()).unwrap();
    overwriter.overwrite_current("cinq".as_bytes()).unwrap();

    let available_args = overwriter.available_paths();
    assert_eq!(available_args.len(), 5);

    std::fs::remove_file(&available_args[1]).unwrap();
    std::fs::remove_file(&available_args[3]).unwrap();

    let result0 = overwriter.get_by_path(&available_args[0]);
    let result1 = overwriter.get_by_path(&available_args[1]);
    let result2 = overwriter.get_by_path(&available_args[2]);
    let result3 = overwriter.get_by_path(&available_args[3]);
    let result4 = overwriter.get_by_path(&available_args[4]);

    assert!(result0.is_ok());
    assert!(result1.is_err());
    assert!(result2.is_ok());
    assert!(result3.is_err());
    assert!(result4.is_ok());

    let current = String::from_utf8(result0.unwrap()).unwrap();
    let old2 = String::from_utf8(result2.unwrap()).unwrap();
    let old4 = String::from_utf8(result4.unwrap()).unwrap();

    assert_eq!(current, "cinq");
    assert_eq!(old2, "trois");
    assert_eq!(old4, "un");
}

#[test]
fn should_get_file_contents_by_index() {
    let test_dir = ris_util::prep_test_dir!();
    let overwriter = FallbackFileOverwrite::new(&test_dir, ".test", 10);

    assert!(overwriter.get_by_index(0).is_err());

    overwriter.overwrite_current("un".as_bytes()).unwrap();
    overwriter.overwrite_current("deux".as_bytes()).unwrap();
    overwriter.overwrite_current("trois".as_bytes()).unwrap();
    overwriter.overwrite_current("quatre".as_bytes()).unwrap();
    overwriter.overwrite_current("cinq".as_bytes()).unwrap();

    let current = String::from_utf8(overwriter.get_by_index(0).unwrap()).unwrap();
    let old1 = String::from_utf8(overwriter.get_by_index(1).unwrap()).unwrap();
    let old2 = String::from_utf8(overwriter.get_by_index(2).unwrap()).unwrap();
    let old3 = String::from_utf8(overwriter.get_by_index(3).unwrap()).unwrap();
    let old4 = String::from_utf8(overwriter.get_by_index(4).unwrap()).unwrap();

    assert_eq!(current, "cinq");
    assert_eq!(old1, "quatre");
    assert_eq!(old2, "trois");
    assert_eq!(old3, "deux");
    assert_eq!(old4, "un");
    assert!(overwriter.get_by_index(5).is_err());
}

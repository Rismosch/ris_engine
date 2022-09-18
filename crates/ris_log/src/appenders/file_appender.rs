use std::{
    fs::{DirEntry, File},
    io::{self, BufRead, Error, Write},
    path::{Path, PathBuf},
    time::SystemTime,
};

use chrono::Local;
use ris_data::info::app_info::AppInfo;

use super::i_appender::IAppender;

const LOG_DIRERCTORY_NAME: &str = "logs";
const LOG_FILE_NAME: &str = "current.log";
const OLD_LOG_DIRERCTORY: &str = "old";
const OLD_LOG_COUNT: usize = 10;

pub struct FileAppender {
    log_file: File,
}

impl FileAppender {
    pub fn new(app_info: &AppInfo) -> Box<Self> {
        let (old_log_directory, current_log_path) = construct_paths(app_info);
        create_old_directory(&old_log_directory);
        delete_expired_logs(&old_log_directory);
        move_current_log(&old_log_directory, &current_log_path);
        let log_file = create_new_log_file(&current_log_path);

        let mut appender = Self { log_file };
        let formatted_timestamp = format!("{}", Local::now());
        appender.print(&formatted_timestamp);

        Box::new(appender)
    }
}

impl IAppender for FileAppender {
    fn print(&mut self, message: &str) {
        let result = writeln!(self.log_file, "{}\n", message);
        if let Err(error) = result {
            panic!("could not log message: {}", error);
        }
    }
}

fn construct_paths(app_info: &AppInfo) -> (PathBuf, PathBuf) {
    let mut log_directory = PathBuf::new();
    log_directory.push(&app_info.file.pref_path);
    log_directory.push(LOG_DIRERCTORY_NAME);

    let mut old_log_directory = PathBuf::new();
    old_log_directory.push(log_directory.clone());
    old_log_directory.push(OLD_LOG_DIRERCTORY);

    let mut current_log_path = PathBuf::new();
    current_log_path.push(log_directory);
    current_log_path.push(LOG_FILE_NAME);

    (old_log_directory, current_log_path)
}

fn create_old_directory(old_log_directory: &PathBuf) {
    if !&old_log_directory.exists() {
        if let Err(error) = std::fs::create_dir_all(&old_log_directory) {
            panic!("couldn't create \"{:?}\": {}", &old_log_directory, error);
        };
    }
}

fn delete_expired_logs(old_log_directory: &PathBuf) {
    let entries = match std::fs::read_dir(&old_log_directory) {
        Ok(entries) => entries,
        Err(error) => panic!("couldn't read \"{:?}\": {}", old_log_directory, error),
    };

    let mut sorted_entries: Vec<_> = entries.collect();
    sorted_entries.sort_by(|left, right| {
        let left = get_modified(left);
        let right = get_modified(right);

        right.cmp(&left)
    });

    for entry in sorted_entries.iter().skip(OLD_LOG_COUNT - 1) {
        let entry = match entry {
            Ok(entry) => entry,
            Err(error) => panic!("couldn't read entry: {}", error),
        };

        let metadata = match entry.metadata() {
            Ok(metadata) => metadata,
            Err(error) => panic!("couldn't read metadata  \"{:?}\": {}", entry.path(), error),
        };

        if metadata.is_dir() {
            let _ = std::fs::remove_dir_all(entry.path());
        } else {
            let _ = std::fs::remove_file(entry.path());
        }
    }
}

fn move_current_log(old_log_directory: &Path, current_log_path: &Path) {
    if !current_log_path.exists() {
        return;
    }

    let file = match File::open(current_log_path) {
        Ok(file) => file,
        Err(error) => panic!("couldn't read \"{:?}\": {}", current_log_path, error),
    };

    let mut lines = io::BufReader::new(file).lines();
    let first_line = match lines.next() {
        Some(Ok(line)) => format!("{}.log", line),
        _ => default_old_filename(),
    };

    let source = current_log_path;
    let mut target = old_log_directory.to_path_buf();
    target.push(sanitize(&first_line));

    match std::fs::rename(source, target) {
        Ok(()) => (),
        Err(_) => {
            let mut target = old_log_directory.to_path_buf();
            target.push(sanitize(&default_old_filename()));

            if let Err(error) = std::fs::rename(source, &target) {
                panic!(
                    "couldn't move \"{:?}\" to \"{:?}\": {}",
                    source, target, error
                );
            }
        }
    }
}

fn create_new_log_file(current_log_path: &PathBuf) -> File {
    match File::create(&current_log_path) {
        Ok(file) => file,
        Err(error) => panic!("couldn't create \"{:?}\": {}", current_log_path, error),
    }
}

fn get_modified(entry: &Result<DirEntry, Error>) -> SystemTime {
    let entry = match entry {
        Ok(entry) => entry,
        Err(error) => panic!("couldn't read entry: {}", error),
    };

    let metadata = match entry.metadata() {
        Ok(meta_data) => meta_data,
        Err(error) => panic!("couldn't retreive metadata: {}", error),
    };

    match metadata.modified() {
        Ok(modified) => modified,
        Err(error) => panic!("couldn't retreive modified time: {}", error),
    }
}

fn default_old_filename() -> String {
    format!("{}.log", Local::now())
}

fn sanitize(value: &str) -> String {
    const INVALID_CHARS: [char; 9] = ['\\', '/', ':', '*', '?', '"', '<', '>', '|'];

    let mut value = String::from(value);
    for invalid_char in INVALID_CHARS {
        value = value.replace(invalid_char, "_");
    }

    value
}

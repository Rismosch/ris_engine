use std::{
    fs::{DirEntry, File},
    io::{self, BufRead, Error, Write},
    path::{Path, PathBuf},
    time::SystemTime,
};

use chrono::Local;
use ris_data::info::app_info::AppInfo;
use ris_log::i_appender::IAppender;
use ris_util::unwrap_or_throw;

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
        unwrap_or_throw!(result, "could not log message")
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
        let result = std::fs::create_dir_all(&old_log_directory);
        unwrap_or_throw!(result, "couldn't create \"{:?}\"", &old_log_directory);
    }
}

fn delete_expired_logs(old_log_directory: &PathBuf) {
    let entries = unwrap_or_throw!(
        std::fs::read_dir(&old_log_directory),
        "couldn't read \"{:?}\"",
        old_log_directory
    );

    let mut sorted_entries: Vec<_> = entries.collect();
    sorted_entries.sort_by(|left, right| {
        let left = get_modified(left);
        let right = get_modified(right);

        right.cmp(&left)
    });

    for entry in sorted_entries.iter().skip(OLD_LOG_COUNT - 1) {
        let entry = unwrap_or_throw!(entry, "couldn't read entry");
        let metadata = unwrap_or_throw!(
            entry.metadata(),
            "couldn't read metadata  \"{:?}\"",
            entry.path()
        );

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

    let file = unwrap_or_throw!(
        File::open(current_log_path),
        "couldn't read \"{:?}\"",
        current_log_path
    );

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

            unwrap_or_throw!(
                std::fs::rename(source, &target),
                "couldn't move \"{:?}\" to \"{:?}\"",
                source,
                target
            );
        }
    }
}

fn create_new_log_file(current_log_path: &PathBuf) -> File {
    unwrap_or_throw!(
        File::create(&current_log_path),
        "couldn't create \"{:?}\"",
        current_log_path
    )
}

fn get_modified(entry: &Result<DirEntry, Error>) -> SystemTime {
    let entry = unwrap_or_throw!(entry, "couldn't read entry");
    let metadata = unwrap_or_throw!(entry.metadata(), "couldn't retreive metadata");

    unwrap_or_throw!(metadata.modified(), "couldn't retreive modified time")
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

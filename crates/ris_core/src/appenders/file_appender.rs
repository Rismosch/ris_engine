use std::cmp::Ordering;
use std::fs::DirEntry;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::Error;
use std::io::Write;
use std::path::PathBuf;
use std::time::SystemTime;

use chrono::Local;

use ris_data::info::app_info::AppInfo;
use ris_log::i_appender::IAppender;
use ris_util::ris_error::RisResult;

const LOG_DIRERCTORY_NAME: &str = "logs";
const LOG_FILE_NAME: &str = "current.log";
const OLD_LOG_DIRERCTORY: &str = "old";
const OLD_LOG_COUNT: usize = 10;

pub struct FileAppender {
    log_file: File,
}

impl FileAppender {
    pub fn new(app_info: &AppInfo) -> RisResult<Box<Self>> {
        // construct paths
        let mut log_directory = PathBuf::new();

        log_directory.push(&app_info.file.pref_path);
        log_directory.push(LOG_DIRERCTORY_NAME);

        let mut old_log_directory = PathBuf::new();
        old_log_directory.push(log_directory.clone());
        old_log_directory.push(OLD_LOG_DIRERCTORY);

        let mut current_log_path = PathBuf::new();
        current_log_path.push(log_directory);
        current_log_path.push(LOG_FILE_NAME);

        // create old directory
        if !&old_log_directory.exists() {
            ris_util::unroll!(
                std::fs::create_dir_all(&old_log_directory),
                "failed to create dir all \"{:?}\"", &old_log_directory
            )?;
        }

        // delete expired logs
        let entries = ris_util::unroll!(
            std::fs::read_dir(&old_log_directory),
            "failed to read dir \"{:?}\"",
            old_log_directory
        )?;

        let mut sorted_entries: Vec<_> = entries.collect();
        sorted_entries.sort_by(|left, right| {
            let left = get_modified_time(left);
            let right = get_modified_time(right);

            if left.is_err() && right.is_err() {
                Ordering::Equal
            } else if left.is_err() && right.is_ok() {
                Ordering::Less
            } else if left.is_ok() && right.is_err() {
                Ordering::Greater
            } else if left.is_ok() && right.is_ok() {
                let right = right.unwrap();
                let left = left.unwrap();
                right.cmp(&left)
            } else {
                unreachable!();
            }
        });

        for entry in sorted_entries.iter().skip(OLD_LOG_COUNT - 1) {
            let entry = entry.as_ref().map_err(|e| ris_util::new_err!("failed to read entry: {}", e))?;
            let metadata = ris_util::unroll!(
                entry.metadata(),
                "failed to get entry metadata  \"{:?}\"",
                entry.path()
            )?;

            if metadata.is_dir() {
                let _ = std::fs::remove_dir_all(entry.path());
            } else {
                let _ = std::fs::remove_file(entry.path());
            }
        }

        // move current log
        if current_log_path.exists() {
            let file = ris_util::unroll!(
                File::open(&current_log_path),
                "failed to open file \"{:?}\"",
                current_log_path
            )?;

            let mut lines = io::BufReader::new(file).lines();
            let first_line = match lines.next() {
                Some(Ok(line)) => format!("{}.log", line),
                _ => default_date_filename(),
            };

            let source = &current_log_path;
            let mut target = old_log_directory.to_path_buf();
            target.push(sanitize_path(&first_line));

            match std::fs::rename(source, target) {
                Ok(()) => (),
                Err(_) => {
                    let mut target = old_log_directory.to_path_buf();
                    target.push(sanitize_path(&default_date_filename()));

                    ris_util::unroll!(
                        std::fs::rename(source, &target),
                        "failed to rename \"{:?}\" to \"{:?}\"",
                        source,
                        target
                    )?;
                }
            }
        }

        // create new log file
        let log_file = ris_util::unroll!(
            File::create(&current_log_path),
            "failed to create \"{:?}\"",
            current_log_path
        )?;

        // create appender
        let mut appender = Self { log_file };
        let formatted_timestamp = format!("{}", Local::now());
        appender.print(&formatted_timestamp);

        Ok(Box::new(appender))
    }
}

impl IAppender for FileAppender {
    fn print(&mut self, message: &str) {
        let result = writeln!(self.log_file, "{}\n", message);

        if let Err(e) = result {
            let error_message = format!("failed to log message \"{:?}\"\nerror: {}", message, e);

            let _ = sdl2::messagebox::show_simple_message_box(
                sdl2::messagebox::MessageBoxFlag::ERROR,
                "Log Error",
                &error_message,
                None,
            );

            panic!("{}", error_message);
        }

    }
}

fn get_modified_time(entry: &Result<DirEntry, Error>) -> RisResult<SystemTime> {
    let entry = entry.as_ref().map_err(|e| ris_util::new_err!("failed to read entry: {}", e))?;
    let metadata = ris_util::unroll!(entry.metadata(), "failed to retreive metadata")?;
    ris_util::unroll!(metadata.modified(), "failed to retreive modified time")
}

fn default_date_filename() -> String {
    format!("{}.log", Local::now())
}

fn sanitize_path(value: &str) -> String {
    const INVALID_CHARS: [char; 9] = ['\\', '/', ':', '*', '?', '"', '<', '>', '|'];

    let mut value = String::from(value);
    for invalid_char in INVALID_CHARS {
        value = value.replace(invalid_char, "_");
    }

    value
}

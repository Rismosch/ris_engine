use std::cmp::Ordering;
use std::fs::DirEntry;
use std::fs::File;
use std::io::BufRead;
use std::io::Read;
use std::io::Write;
use std::io::Seek;
use std::path::Path;
use std::path::PathBuf;
use std::time::SystemTime;

use chrono::Local;

use crate::error::RisResult;
use crate as ris_util;

const OLD_FILE_COUNT: usize = 10;

struct FallbackFileAppend{
    current: File,
}

impl FallbackFileAppend {
    pub fn new(directory: &Path, file_extension: &str) -> RisResult<Self> {
        // create paths
        let mut current_path = PathBuf::new();
        current_path.push(directory);
        let filename = format!("current{}", file_extension);
        current_path.push(filename);

        let mut old_directory = PathBuf::new();
        old_directory.push(directory);
        old_directory.push("old");

        // create directories
        ris_util::unroll!(
            std::fs::create_dir_all(&old_directory),
            "failed to create directory \"{:?}\"",
            &old_directory,
        )?;

        // delete expired files
        let entries = ris_util::unroll!(
            std::fs::read_dir(&old_directory),
            "failed to read \"{:?}\"",
            &old_directory,
        )?;

        let mut sorted_entries: Vec<_> = entries.collect();
        sorted_entries.sort_by(|left, right| {
            let left_modified_result = get_modified(left);
            let right_modified_result = get_modified(right);

            match left_modified_result {
                Ok(left_modified) => {
                    match right_modified_result {
                        Ok(right_modified) => right_modified.cmp(&left_modified),
                        Err(_) => Ordering::Less,
                    }
                },
                Err(_) => {
                    match right_modified_result {
                        Ok(_right_modified) => Ordering::Greater,
                        Err(_) => Ordering::Equal,
                    }
                },
            }
        });
        
        for entry_result in sorted_entries.iter().skip(OLD_FILE_COUNT - 1) {
            let entry = entry_result
                .as_ref()
                .map_err(|e| ris_util::new_err!("failed to read entry: {}", e))?;
            let metadata = ris_util::unroll!(entry.metadata(), "failed to get metadata")?;

            if metadata.is_dir() {
                let _ = std::fs::remove_dir_all(entry.path());
            } else {
                let _ = std::fs::remove_file(entry.path());
            }
        }

        // move current file
        if current_path.exists() {
            let file = ris_util::unroll!(
                File::open(&current_path),
                "failed to open file \"{:?}\"",
                current_path,
            )?;

            let mut lines = std::io::BufReader::new(file).lines();
            let previous_filename_unsanitized = match lines.next() {
                Some(Ok(line)) => line,
                _ => format!("{}", Local::now()),
            };
            let previous_filename_without_extension = sanitize(&previous_filename_unsanitized);

            let mut previous_path = PathBuf::new();
            previous_path.push(&old_directory);
            let previous_filename = format!(
                "{}{}",
                previous_filename_without_extension,
                file_extension,
            );
            previous_path.push(previous_filename);
            let mut counter = 0;
            while previous_path.exists(){
                counter += 1;

                previous_path = PathBuf::new();
                previous_path.push(&old_directory);
                let previous_filename = format!(
                    "{}({}){}",
                    previous_filename_without_extension,
                    counter,
                    file_extension,
                );
                previous_path.push(previous_filename);
            }

            ris_util::unroll!(
                std::fs::rename(&current_path, &previous_path),
                "failed to rename \"{:?}\" to \"{:?}\"",
                current_path,
                previous_path,
            )?;
        }

        // create current file
        let current_file = ris_util::unroll!(
            File::create(&current_path),
            "failed to create \"{:?}\"",
            current_path,
        )?;

        // create self
        Ok(Self {
            current: current_file,
        })
    }

    pub fn current(&self) -> &(impl Read + Seek) {
        &self.current
    }

    pub fn current_as_mut(&mut self) -> &mut (impl Read + Write + Seek) {
        &mut self.current
    }
}

struct FallbackFileOverwrite{}

impl FallbackFileOverwrite {
    pub fn new(directory: &Path, file_extension: &str) -> RisResult<Self> {
        panic!();
    }

    //pub fn open_latest() -> Option<&(impl Read + Seek)> {

    //}

    //pub fn overwrite() {

    //}
}

fn get_modified(entry: &Result<DirEntry, std::io::Error>) -> RisResult<SystemTime> {
    let entry = ris_util::unroll!(
        entry,
        "failed to read entry",
    )?;

    let metadata = ris_util::unroll!(
        entry.metadata(),
        "failed to read metadata",
    )?;

    ris_util::unroll!(
        metadata.modified(),
        "failed to get modified",
    )
}

fn sanitize(value: &str) -> String {
    const INVALID_CHARS: [char; 9] = ['\\', '/', ':', '*', '?', '"', '<', '>', '|'];

    let mut value = String::from(value);
    for invalid_char in INVALID_CHARS {
        value = value.replace(invalid_char, "_");
    }

    value
}

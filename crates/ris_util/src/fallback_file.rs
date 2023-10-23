use std::cmp::Ordering;
use std::fs::DirEntry;
use std::fs::File;
use std::io::BufRead;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::time::SystemTime;

use chrono::Local;

use crate::error::RisResult;
use crate as ris_util;

pub const OLD_FILE_COUNT: usize = 10;

pub struct FallbackFileAppend{
    current: File,
}

impl FallbackFileAppend {
    pub fn new(directory: &Path, file_extension: &str) -> RisResult<Self> {
        let (current_path, old_directory) = generate_paths(directory, file_extension);
        create_directories(&old_directory)?;
        delete_expired_files(&old_directory)?;
        move_current_file(&current_path, &old_directory, file_extension)?;
        let current = create_current_file(&current_path)?;

        Ok(Self {current})
    }

    pub fn current(&mut self) -> &mut File {
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

fn generate_paths(directory: &Path, file_extension: &str) -> (PathBuf, PathBuf){
    let mut current_path = PathBuf::new();
    current_path.push(directory);
    let filename = format!("current{}", file_extension);
    current_path.push(filename);

    let mut old_directory = PathBuf::new();
    old_directory.push(directory);
    old_directory.push("old");

    (current_path, old_directory)
}

fn create_directories(old_directory: &Path) -> RisResult<()> {
    ris_util::unroll!(
        std::fs::create_dir_all(old_directory),
        "failed to create directory \"{:?}\"",
        old_directory,
    )
}

fn delete_expired_files(old_directory: &Path) -> RisResult<()> {
    let entries = ris_util::unroll!(
        std::fs::read_dir(old_directory),
        "failed to read \"{:?}\"",
        old_directory,
    )?;

    let mut sorted_entries: Vec<_> = entries.collect();
    sorted_entries.sort_by(|left, right| {
        match left {
            Ok(left) => match right {
                Ok(right) => right.path().cmp(&left.path()),
                Err(_) => Ordering::Less,
            },
            Err(_) => match right {
                Ok(_right) => Ordering::Greater,
                Err(_) => Ordering::Equal,
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

    Ok(())
}

fn move_current_file(current_path: &Path, old_directory: &Path, file_extension: &str) -> RisResult<()> {
    if !current_path.exists() {
        return Ok(());
    }

    let file = ris_util::unroll!(
        File::open(current_path),
        "failed to open file \"{:?}\"",
        current_path,
    )?;

    let mut lines = std::io::BufReader::new(file).lines();
    let previous_filename_unsanitized = match lines.next() {
        Some(Ok(line)) => line,
        _ => format!("{}", Local::now()),
    };
    let previous_filename_without_extension = crate::path::sanitize(&previous_filename_unsanitized, true);

    let mut previous_path = PathBuf::new();
    previous_path.push(old_directory);
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
        previous_path.push(old_directory);
        let previous_filename = format!(
            "{}({}){}",
            previous_filename_without_extension,
            counter,
            file_extension,
        );
        previous_path.push(previous_filename);
    }

    ris_util::unroll!(
        std::fs::rename(current_path, &previous_path),
        "failed to rename \"{:?}\" to \"{:?}\"",
        current_path,
        previous_path,
    )?;

    Ok(())
}

fn create_current_file(current_path: &Path) -> RisResult<File> {
    let mut current_file = ris_util::unroll!(
        File::create(current_path),
        "failed to create \"{:?}\"",
        current_path,
    )?;

    ris_util::unroll!(
        writeln!(current_file, "{}\n", Local::now().to_rfc2822()),
        "failed to write timestamp into current file",
    )?;

    Ok(current_file)
}

fn get_modified(entry: &Result<DirEntry, std::io::Error>) -> RisResult<SystemTime> {
    let entry = entry.as_ref().map_err(|e| ris_util::new_err!(
        "failed to read entry: {}",
        e
    ))?;

    let metadata = ris_util::unroll!(
        entry.metadata(),
        "failed to read metadata",
    )?;

    ris_util::unroll!(
        metadata.modified(),
        "failed to get modified",
    )
}


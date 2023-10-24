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

pub struct FallbackFileAppend{
    current_file: File,
}

impl FallbackFileAppend {
    pub fn new(directory: &Path, file_extension: &str, old_file_count: usize) -> RisResult<Self> {
        let (current_path, old_directory) = generate_paths(directory, file_extension);
        create_directories(&old_directory)?;
        delete_expired_files(&old_directory, old_file_count)?;
        move_current_file(&current_path, &old_directory, file_extension)?;
        let current_file = create_current_file(&current_path)?;

        Ok(Self {current_file})
    }

    pub fn current(&mut self) -> &mut File {
        &mut self.current_file
    }
}

struct FallbackFileOverwrite{
    directory: PathBuf,
    current_path: PathBuf,
    old_directory: PathBuf,
    file_extension: String,
    old_file_count: usize,
}

impl FallbackFileOverwrite {
    pub fn new(directory: &Path, file_extension: &str, old_file_count: usize) -> Self {
        let directory = directory.to_path_buf();
        let file_extension = file_extension.to_string();
        let (current_path, old_directory) = generate_paths(&directory, &file_extension);

        Self{
            directory,
            current_path,
            old_directory,
            file_extension,
            old_file_count,
        }
    }

    pub fn overwrite_current(&self, buf: &[u8]) -> RisResult<()> {
        create_directories(&self.old_directory)?;
        delete_expired_files(&self.old_directory, self.old_file_count)?;
        move_current_file(&self.current_path, &self.old_directory, &self.file_extension)?;
        let mut current_file = create_current_file(&self.current_path)?;

        let written_bytes = ris_util::unroll!(
            current_file.write(buf),
            "failed to write current file",
        )?;
        if written_bytes != buf.len() {
            ris_util::result_err!(
                "failed to write to current file. expected to write {} bytes but actually wrote {}",
                buf.len(),
                written_bytes,
            )
        } else {
            Ok(())
        }
    }

    pub fn available_paths(&self) -> Vec<PathBuf> {
        let mut result = Vec::new();
        
        if self.current_path.exists() {
            result.push(self.current_path.clone());
        }

        if let Ok(mut sorted_entries) = get_sorted_entries(&self.old_directory) {
            result.append(&mut sorted_entries);
        }

        result
    }

    pub fn get(path: &Path) -> Option<Vec<u8>> {
        panic!()
    }
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

fn delete_expired_files(old_directory: &Path, old_file_count: usize) -> RisResult<()> {
    let sorted_entries = get_sorted_entries(old_directory)?;
    
    for entry in sorted_entries.iter().skip(old_file_count - 1) {
        let metadata = ris_util::unroll!(entry.metadata(), "failed to get metadata")?;

        if metadata.is_dir() {
            let _ = std::fs::remove_dir_all(entry);
        } else {
            let _ = std::fs::remove_file(entry);
        }
    }

    Ok(())
}

fn get_sorted_entries(directory: &Path) -> RisResult<Vec<PathBuf>> {
    let entries = ris_util::unroll!(
        std::fs::read_dir(directory),
        "failed to read \"{:?}\"",
        directory,
    )?;

    let mut result: Vec<_> = entries
        .filter(|x| x.is_ok())
        .map(|x| x.expect("somehow, x is Err, despite being filtered out previously").path())
        .collect();

    result.sort_by(|left, right| {
        right.cmp(&left)
    });

    Ok(result)
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


use std::fs::File;
use std::io::BufRead;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

use chrono::DateTime;
use chrono::Local;

use ris_error::RisResult;

use crate as ris_util;

pub struct FallbackFileAppend {
    current_file: File,
}

impl FallbackFileAppend {
    pub fn new(directory: &Path, file_extension: &str, old_file_count: usize) -> RisResult<Self> {
        let (current_path, old_directory) = generate_paths(directory, file_extension);
        create_directories(&old_directory)?;
        delete_expired_files(&old_directory, old_file_count)?;
        move_current_file(&current_path, &old_directory, file_extension)?;
        let current_file = create_current_file(&current_path)?;

        Ok(Self { current_file })
    }

    pub fn current(&mut self) -> &mut File {
        &mut self.current_file
    }
}

pub struct FallbackFileOverwrite {
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

        Self {
            current_path,
            old_directory,
            file_extension,
            old_file_count,
        }
    }

    pub fn overwrite_current(&self, buf: &[u8]) -> RisResult<()> {
        create_directories(&self.old_directory)?;
        delete_expired_files(&self.old_directory, self.old_file_count)?;
        move_current_file(
            &self.current_path,
            &self.old_directory,
            &self.file_extension,
        )?;
        let mut current_file = create_current_file(&self.current_path)?;

        let written_bytes =
            ris_error::unroll!(current_file.write(buf), "failed to write current file",)?;
        if written_bytes != buf.len() {
            ris_error::new_result!(
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

    pub fn get_by_path(&self, path: &Path) -> Option<Vec<u8>> {
        match File::open(path) {
            Ok(mut file) => match read_file_and_strip_date(&mut file) {
                Ok(bytes) => Some(bytes),
                Err(_) => None,
            },
            Err(_) => None,
        }
    }

    pub fn get_by_index(&self, index: usize) -> Option<Vec<u8>> {
        let available_paths = self.available_paths();
        let path_option = available_paths.get(index);
        match path_option {
            Some(path_buf) => self.get_by_path(path_buf),
            None => None,
        }
    }
}

fn generate_paths(directory: &Path, file_extension: &str) -> (PathBuf, PathBuf) {
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
    ris_error::unroll!(
        std::fs::create_dir_all(old_directory),
        "failed to create directory \"{:?}\"",
        old_directory,
    )
}

fn delete_expired_files(old_directory: &Path, old_file_count: usize) -> RisResult<()> {
    let sorted_entries = get_sorted_entries(old_directory)?;

    for entry in sorted_entries.iter().skip(old_file_count - 1) {
        let metadata = ris_error::unroll!(entry.metadata(), "failed to get metadata")?;

        if metadata.is_dir() {
            let _ = std::fs::remove_dir_all(entry);
        } else {
            let _ = std::fs::remove_file(entry);
        }
    }

    Ok(())
}

fn get_sorted_entries(directory: &Path) -> RisResult<Vec<PathBuf>> {
    let entries = ris_error::unroll!(
        std::fs::read_dir(directory),
        "failed to read \"{:?}\"",
        directory,
    )?;

    let mut result: Vec<_> = entries
        .filter(|x| x.is_ok())
        .map(|x| {
            x.expect("somehow, x is Err, despite being filtered out previously")
                .path()
        })
        .collect();

    result.sort_by(|left, right| right.cmp(left));

    Ok(result)
}

fn move_current_file(
    current_path: &Path,
    old_directory: &Path,
    file_extension: &str,
) -> RisResult<()> {
    if !current_path.exists() {
        return Ok(());
    }

    let file = ris_error::unroll!(
        File::open(current_path),
        "failed to open file \"{:?}\"",
        current_path,
    )?;

    let mut lines = std::io::BufReader::new(file).lines();
    let previous_filename_unsanitized = match lines.next() {
        Some(Ok(line)) => line,
        _ => format!("{}", Local::now()),
    };
    let previous_filename_without_extension =
        crate::path::sanitize(&previous_filename_unsanitized, true);

    let mut previous_path = PathBuf::new();
    previous_path.push(old_directory);
    let previous_filename = format!("{}{}", previous_filename_without_extension, file_extension,);
    previous_path.push(previous_filename);

    let attempts = 100;
    for _ in 0..attempts {
        if !previous_path.exists() {
            break;
        }

        std::thread::sleep(std::time::Duration::from_millis(1));

        previous_path = PathBuf::new();
        previous_path.push(old_directory);
        let new_previous_filename = format!("{}{}", Local::now().to_rfc3339(), file_extension);
        let sanitized_new_previous_filename = crate::path::sanitize(&new_previous_filename, true);
        previous_path.push(sanitized_new_previous_filename);
    }

    if previous_path.exists() {
        ris_error::new_result!("failed to generate a unique old filename")
    } else {
        ris_error::unroll!(
            std::fs::rename(current_path, &previous_path),
            "failed to rename \"{:?}\" to \"{:?}\"",
            current_path,
            previous_path,
        )?;

        Ok(())
    }
}

fn create_current_file(current_path: &Path) -> RisResult<File> {
    let mut current_file = ris_error::unroll!(
        File::create(current_path),
        "failed to create \"{:?}\"",
        current_path,
    )?;

    ris_error::unroll!(
        writeln!(current_file, "{}\n", Local::now().to_rfc3339()),
        "failed to write timestamp into current file",
    )?;

    Ok(current_file)
}

fn read_file_and_strip_date(file: &mut File) -> RisResult<Vec<u8>> {
    let file_size = crate::seek!(file, SeekFrom::End(0))?;

    let mut buf = vec![0u8; file_size as usize];
    crate::seek!(file, SeekFrom::Start(0))?;
    crate::read!(file, buf)?;

    let mut first_new_line = None;
    let mut second_new_line = None;
    for (i, char) in buf.iter().enumerate().take(file_size as usize) {
        if *char != b'\n' {
            continue;
        }

        if first_new_line.is_none() {
            first_new_line = Some(i);
        } else {
            second_new_line = Some(i);
            break;
        }
    }

    match (first_new_line, second_new_line) {
        (Some(first_new_line), Some(second_new_line)) => {
            // expect the second line to be empty
            if first_new_line + 1 != second_new_line {
                return Ok(buf);
            }

            // expect the first line to be a string
            let mut first_line_buf = vec![0u8; first_new_line];
            crate::seek!(file, SeekFrom::Start(0))?;
            crate::read!(file, first_line_buf)?;
            let first_line_string = String::from_utf8(first_line_buf);
            match first_line_string {
                Ok(date_string) => {
                    // expect first line to be a valid date
                    let date = DateTime::parse_from_rfc3339(&date_string);
                    if date.is_err() {
                        return Ok(buf);
                    }

                    // first two lines are as expected, we can strip them away
                    let content_addr = (second_new_line + 1) as u64;
                    let content_len = file_size - content_addr;
                    let mut content = vec![0; content_len as usize];
                    crate::seek!(file, SeekFrom::Start(content_addr))?;
                    crate::read!(file, content)?;

                    Ok(content)
                }
                Err(_) => Ok(buf),
            }
        }
        _ => Ok(buf),
    }
}

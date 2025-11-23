use std::io::BufRead;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

use ris_error::prelude::*;
use ris_log::counter::Counter;

use crate::FatPtr;
use crate::path::SanitizeInfo;

pub struct FallbackFileAppend {
    current_file: std::fs::File,
}

impl FallbackFileAppend {
    pub fn new(directory: &Path, file_extension: &str, old_file_count: usize) -> RisResult<Self> {
        let (current_path, old_directory) = generate_paths(directory, file_extension);
        std::fs::create_dir_all(&old_directory)?;
        delete_expired_files(&old_directory, old_file_count)?;
        let counter = move_current_file(&current_path, &old_directory, file_extension)?;
        let current_file = create_current_file(&current_path, counter)?;

        Ok(Self { current_file })
    }

    pub fn current(&mut self) -> &mut std::fs::File {
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
        std::fs::create_dir_all(&self.old_directory)?;
        delete_expired_files(&self.old_directory, self.old_file_count)?;
        let counter = move_current_file(
            &self.current_path,
            &self.old_directory,
            &self.file_extension,
        )?;
        let mut current_file = create_current_file(&self.current_path, counter)?;

        let written_bytes = current_file.write(buf)?;
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

    pub fn get_by_path(&self, path: &Path) -> RisResult<Vec<u8>> {
        let FileStructure { counter: _, p_content } = parse_file_header(path)?;
        let mut file = std::fs::File::open(path)?;
        let bytes = crate::io::read_at(&mut file, p_content)?;
        Ok(bytes)
    }

    pub fn get_by_index(&self, index: usize) -> RisResult<Vec<u8>> {
        let available_paths = self.available_paths();
        let path = available_paths.get(index).into_ris_error()?;
        self.get_by_path(path)
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

fn delete_expired_files(old_directory: &Path, old_file_count: usize) -> RisResult<()> {
    let sorted_entries = get_sorted_entries(old_directory)?;

    for entry in sorted_entries.iter().skip(old_file_count - 1) {
        let metadata = entry.metadata()?;

        if metadata.is_dir() {
            let _ = std::fs::remove_dir_all(entry);
        } else {
            let _ = std::fs::remove_file(entry);
        }
    }

    Ok(())
}

fn get_sorted_entries(directory: &Path) -> RisResult<Vec<PathBuf>> {
    let entries = std::fs::read_dir(directory)?;

    let mut result: Vec<_> = entries
        .filter(|x| x.is_ok())
        .map(|x| {
            let dir_entry = ris_error::unwrap!(
                x,
                "somehow, x is Err, despite being filtered out previously",
            );
            dir_entry.path()
        })
        .collect();

    result.sort_by(|left, right| {
        let left = match parse_file_header(left){
            Ok(FileStructure { counter: Some(counter), p_content: _ }) => counter,
            _ => Counter::MAX,
        };
        let right = match parse_file_header(right){
            Ok(FileStructure { counter: Some(counter), p_content: _ }) => counter,
            _ => Counter::MAX,
        };
        
        right.cmp(&left)
    });

    Ok(result)
}

fn move_current_file(
    current_path: &Path,
    old_directory: &Path,
    file_extension: &str,
) -> RisResult<Counter> {
    if !current_path.exists() {
        return Ok(Counter::default());
    }

    let file = std::fs::File::open(current_path)?;

    let mut lines = std::io::BufReader::new(file).lines();
    let previous_counter = match lines.next() {
        Some(Ok(line)) => match line.trim().parse::<u32>() {
            Ok(n) => Counter::from_raw(n),
            _ => Counter::MAX,
        }
        _ => Counter::MAX,
    };
    let previous_filename_without_extension = previous_counter.raw().to_string();

    let mut previous_path = PathBuf::new();
    previous_path.push(old_directory);
    let previous_filename = format!("{}{}", previous_filename_without_extension, file_extension);
    previous_path.push(previous_filename);

    let attempts = 100;
    for i in 0..attempts {
        if !previous_path.exists() {
            break;
        }

        std::thread::sleep(std::time::Duration::from_millis(1));

        previous_path = PathBuf::new();
        previous_path.push(old_directory);

        let post_fix = if i == 0 {
            String::new()
        } else {
            format!("({})", i)
        };

        let new_previous_filename = format!(
            "{}{}{}",
            previous_counter.raw(),
            post_fix,
            file_extension,
        );
        let sanitized_new_previous_filename = crate::path::sanitize(
            &new_previous_filename,
            SanitizeInfo::RemoveInvalidCharsAndSlashes,
        );
        previous_path.push(sanitized_new_previous_filename);
    }

    if previous_path.exists() {
        ris_error::new_result!("failed to generate a unique old filename")
    } else {
        std::fs::rename(current_path, &previous_path)?;

        let mut new_counter = previous_counter;
        new_counter.increase();
        Ok(new_counter)
    }
}

fn create_current_file(current_path: &Path, counter: Counter) -> RisResult<std::fs::File> {
    let mut current_file = std::fs::File::create(current_path)?;
    writeln!(current_file, "{}\n", counter.raw())?;
    Ok(current_file)
}

struct FileStructure {
    counter: Option<Counter>,
    p_content: FatPtr,
}

fn parse_file_header(path: impl AsRef<Path>) -> RisResult<FileStructure> {
    let mut file = std::fs::File::open(path.as_ref())?;
    let file = &mut file;

    let mut begin = 0u64;
    let end = crate::seek(file, SeekFrom::End(0))?;

    // max u32 `4294967295` has 10 character. plus two line breaks
    // (assuming `\r\n`) gives us 14 character thus a buffer of size
    // 2^5=16 should be enough to check whether the file header is
    // correctly formatted or not
    let mut buf = vec![0u8; 16];
    file.seek(SeekFrom::Start(0))?;
    let read_bytes = file.read(&mut buf)?;

    let mut first_line_break_index = None;
    let mut second_line_break_index = None;
    for (i, char) in buf.iter().enumerate().take(read_bytes as usize) {
        if *char != b'\n' {
            continue;
        }

        if first_line_break_index.is_none() {
            first_line_break_index = Some(i);
        } else {
            second_line_break_index = Some(i);
            begin = i as u64 + 1;
            break;
        }
    }

    let mut result = FileStructure{
        counter: None,
        p_content: FatPtr::begin_end(begin, end)?,
    };

    let (
        Some(first_line_break_index),
        Some(second_line_break_index),
    ) = (first_line_break_index, second_line_break_index) else {
        return Ok(result)
    };

    // expect the second line to be empty
    if first_line_break_index + 1 != second_line_break_index {
        return Ok(result);
    }

    // expect the first line to be a string
    let first_line_bytes = &buf[0..first_line_break_index];
    let first_line = str::from_utf8(first_line_bytes);
    let Ok(number_string) = first_line else {
        return Ok(result);
    };

    // expect first line to be an unsigned integer
    let Ok(integer) = number_string.parse::<u32>() else {
        return Ok(result);
    };

    // first two lines are as expected, we can strip them away
    result.counter = Some(Counter::from_raw(integer));
    result.p_content = FatPtr::begin_end(
        second_line_break_index as u64 + 1,
        end,
    )?;

    Ok(result)
}

use std::cmp::Ordering;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::io::Seek;
use std::path::Path;
use std::path::PathBuf;

use crate::error::RisResult;
use crate as ris_util;

const OLD_FILE_COUNT: usize = 10;

pub struct FallbackFile{
    current: File,
    previous: Option<File>,
}

impl FallbackFile {
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
            left.
            //match left {
            //    Ok(left_entry) => {
            //        match right {
            //            Ok(right_entry) => {

            //            },
            //            Err(_) => Ordering::Greater,
            //        }
            //    },
            //    Err(_e) => {
            //        match right {
            //            Ok(right_entry) => Ordering::Less,
            //            Err(_) => Ordering::Equal,
            //        }
            //    },
            //}
            panic!()
        });
        
        for entry in sorted_entries.iter().skip(OLD_FILE_COUNT - 1) {

        }

        // move current file

        // create new file

        panic!();
    }

    pub fn current(&self) -> &(impl Read + Seek) {
        &self.current
    }

    pub fn current_as_mut(&mut self) -> &mut (impl Read + Write + Seek) {
        &mut self.current
    }

    pub fn previous(&self) -> Option<&(impl Read + Seek)> {
        self.previous.as_ref()
    }
}


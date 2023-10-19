use std::path::Path;
use std::path::PathBuf;

pub struct FallbackFileIO{
    directory: PathBuf,
    file_extension: String,
    old_file_count: usize,
}

impl FallbackFileIO {
    pub fn new(directory: &Path, file_extension: &str, old_file_count: usize) -> Self {
        Self{
            directory: directory.to_path_buf(),
            file_extension: file_extension.to_string(),
            old_file_count,
        }
    }

    pub fn open_current(&self) {

    }

    pub fn write_current(){

    }

    pub fn read_old(){

    }

}

fn current_filepath() -> PathBuf {
    panic!();
    //let mut result = PathBuf::new();
    //result.push(&app_info.file.pref_path);
    //result.push(SETTINGS_DIRECTORY_NAME);

    //result
}

fn old_directory() -> PathBuf {
    panic!();
    //let mut result = PathBuf::new();
    //result.push(settings_directory(app_info));
    //result.push(OLD_SETTINGS_DIRECTORY_NAME);
    //
    //result
}


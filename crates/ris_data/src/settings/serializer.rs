use std::io::Read;
use std::io::Seek;
use std::io::Write;
use std::path::PathBuf;

use ris_util::error::RisResult;

use crate::info::app_info::AppInfo;
use crate::settings::Settings;

pub const SETTINGS_DIRECTORY_NAME: &str = "settings";
pub const OLD_SETTINGS_DIRECTORY_NAME: &str = "old";
pub const SETTINGS_FILE_NAME: &str = "current.ris_settings";

pub fn serialize(settings: &Settings, app_info: &AppInfo) -> RisResult<()> {
    panic!("not implemented")
}

pub fn deserialize(app_info: &AppInfo) -> Option<Settings> {
    panic!("not implemented")
}

fn settings_directory(app_info: &AppInfo) -> PathBuf {
    let mut result = PathBuf::new();
    result.push(&app_info.file.pref_path);
    result.push(SETTINGS_DIRECTORY_NAME);

    result
}

fn old_settings_directory(app_info: &AppInfo) -> PathBuf {
    let mut result = PathBuf::new();
    result.push(settings_directory(app_info));
    result.push(OLD_SETTINGS_DIRECTORY_NAME);

    result
}

fn settings_filepath(app_info: &AppInfo) -> PathBuf {
    let mut result = PathBuf::new();
    result.push(settings_directory(app_info));
    result.push(SETTINGS_FILE_NAME);

    result
}

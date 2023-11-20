use std::io::Cursor;
use std::io::Read;
use std::io::Seek;
use std::io::Write;
use std::path::PathBuf;

use ris_util::error::RisResult;
use ris_util::fallback_file::FallbackFileOverwrite;

use crate::info::app_info::AppInfo;
use crate::settings::Settings;

pub const SETTINGS_DIRECTORY_NAME: &str = "settings";
pub const OLD_SETTINGS_DIRECTORY_NAME: &str = "old";
pub const SETTINGS_FILE_NAME: &str = "current.ris_settings";

pub struct SettingsSerializer {
    fallback_file: FallbackFileOverwrite,
}

impl SettingsSerializer {
    pub fn new(app_info: &AppInfo) -> Self {
        let mut settings_dir = PathBuf::new();
        settings_dir.push(&app_info.file.pref_path);
        settings_dir.push("settings");

        let extension = ".ris_settings";
        let old_count = 10;

        let fallback_file = FallbackFileOverwrite::new(&settings_dir, extension, old_count);

        Self { fallback_file }
    }

    pub fn serialize(&self, settings: &Settings) -> RisResult<()> {
        let bytes = write_bytes(settings);
        self.fallback_file.overwrite_current(&bytes)
    }

    pub fn deserialize(&self) -> Option<Settings> {
        for availale_path in self.fallback_file.available_paths() {
            if let Some(bytes) = self.fallback_file.get_by_path(&availale_path) {
                if let Ok(settings) = read_bytes(&bytes) {
                    return Some(settings);
                }
            }
        }

        None
    }
}

fn write_bytes(settings: &Settings) -> Vec<u8> {
    let mut result = Vec::new();
    let mut cursor = Cursor::new(&mut result);

    result
}

fn read_bytes(bytes: &[u8]) -> RisResult<Settings> {
    ris_util::result_err!("not implemented")
}

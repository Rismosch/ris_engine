pub mod job_settings;
pub mod serializer;

use std::path::PathBuf;

use job_settings::JobSettings;
use ris_util::error::RisResult;

use crate::info::app_info::AppInfo;
use crate::settings::serializer;

pub struct Settings{
    pub job: JobSettings,
}

impl Settings {
    pub fn new(app_info: &AppInfo) -> RisResult<Self> {
        let mut settings_directory = PathBuf::new();
        settings_directory.push(&app_info.file.pref_path);
        settings_directory.push(serializer::SETTINGS_DIRECTORY_NAME);

        let mut settings_path = PathBuf::new();
        settings_path.push(&settings_directory);
        settings_path.push(serializer::SETTINGS_FILE_NAME);

        if !&settings_directory.exists() {
            ris_util::unroll!(
                std::fs::create_dir_all(settings_directory),
                "failed to create dir \"{:?}\"",
                settings_directory
            )?;
        }

        std::fs::File::open();

        let settings = match serializer::deserialize() {
            Some(settings) => settings,
            None => {
                panic!("not implemented yet");
            }
        };

        Ok(settings)
    }
}

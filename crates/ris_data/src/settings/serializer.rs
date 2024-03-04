use std::path::PathBuf;

use ris_error::RisResult;
use ris_file::fallback_file::FallbackFileOverwrite;

use crate::info::app_info::AppInfo;
use crate::settings::key;
use crate::settings::ris_yaml::RisYaml;
use crate::settings::Settings;

//pub const DEFAULT: &str = "default";
pub const DIRECTORY_NAME: &str = "settings";
pub const EXTENSION: &str = ".ris_yaml";

pub struct SettingsSerializer {
    fallback_file: FallbackFileOverwrite,
}

impl SettingsSerializer {
    pub fn new(app_info: &AppInfo) -> Self {
        let mut settings_dir = PathBuf::new();
        settings_dir.push(&app_info.file.pref_path);
        settings_dir.push(DIRECTORY_NAME);

        let fallback_file = FallbackFileOverwrite::new(&settings_dir, EXTENSION, 10);

        Self { fallback_file }
    }

    pub fn serialize(&self, settings: &Settings) -> RisResult<()> {
        ris_log::debug!("serializing settings...");

        let bytes = write_bytes(settings);
        self.fallback_file.overwrite_current(&bytes?)?;

        ris_log::debug!("settings serialized!");

        Ok(())
    }

    pub fn deserialize(&self, app_info: &AppInfo) -> Option<Settings> {
        ris_log::debug!("deserializing settings...");

        for available_path in self.fallback_file.available_paths() {
            if let Some(bytes) = self.fallback_file.get_by_path(&available_path) {
                match read_bytes(&bytes, app_info) {
                    Ok(settings) => {
                        ris_log::debug!("settings deserialized!");
                        return Some(settings);
                    }
                    Err(error) => {
                        ris_log::warning!(
                            "failed to deserialize \"{:?}\": {}",
                            available_path,
                            error
                        );
                    }
                }
            }
        }

        ris_log::debug!("no valid settings found");

        None
    }
}

fn write_bytes(settings: &Settings) -> RisResult<Vec<u8>> {
    let mut yaml = RisYaml::default();

    yaml.add_comment("jobs");
    yaml.add_key_value(key::JOB_WORKERS, &settings.job.get_workers().to_string());
    yaml.add_empty();

    let string = yaml.to_string()?;

    let bytes = string.as_bytes().to_vec();
    Ok(bytes)
}

fn read_bytes(bytes: &[u8], app_info: &AppInfo) -> RisResult<Settings> {
    let string = String::from_utf8(bytes.to_vec())?;

    let mut result = Settings::new(app_info);
    let yaml = RisYaml::try_from(string.as_str())?;

    for (i, entry) in yaml.entries.iter().enumerate() {
        let (key, value) = match entry.key_value.as_ref() {
            Some(key_value) => key_value,
            None => continue,
        };

        match key.as_str() {
            key::JOB_WORKERS => result.job.set_workers(value.parse()?),
            _ => return ris_error::new_result!("unkown key at line {}", i),
        }
    }

    Ok(result)
}


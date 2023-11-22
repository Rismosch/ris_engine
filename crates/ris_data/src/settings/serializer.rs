use std::fmt::Display;
use std::path::PathBuf;
use std::str::FromStr;

use ris_util::error::RisResult;
use ris_util::fallback_file::FallbackFileOverwrite;

use crate::info::app_info::AppInfo;
use crate::settings::key;
use crate::settings::ris_yaml::error_on_line;
use crate::settings::ris_yaml::RisYaml;
use crate::settings::ris_yaml::RisYamlEntry;
use crate::settings::Settings;

pub const DEFAULT: &str = "default";
pub const DIRECTORY_NAME: &str = "settings";
pub const EXTENSION: &str = ".ris_settings";

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
        let bytes = write_bytes(settings);
        self.fallback_file.overwrite_current(&bytes?)
    }

    pub fn deserialize(&self) -> Option<Settings> {
        for available_path in self.fallback_file.available_paths() {
            if let Some(bytes) = self.fallback_file.get_by_path(&available_path) {
                match read_bytes(&bytes) {
                    Ok(settings) => return Some(settings),
                    Err(error) => ris_log::warning!("failed to deserialize \"{:?}\": {}", available_path, error),
                }
            }
        }

        None
    }
}

fn write_bytes(settings: &Settings) -> RisResult<Vec<u8>> {
    let mut yaml = RisYaml::default();

    yaml.add_comment("jobs");
    yaml.add_key_value(key::JOB_WORKERS, compose(&settings.job.workers));
    yaml.add_empty();

    let string = ris_util::unroll!(
        yaml.to_string(),
        "failed to serialize yaml",
    )?;

    let bytes = string.as_bytes().to_vec();
    Ok(bytes)
}

fn read_bytes(bytes: &[u8]) -> RisResult<Settings> {
    let string = ris_util::unroll!(String::from_utf8(bytes.to_vec()), "failed to parse bytes",)?;

    let mut result = Settings::default();
    let yaml = RisYaml::from(&string)?;

    for (i, entry) in yaml.entries.iter().enumerate() {
        let line = i + 1;

        let (key, value) = match entry.key_value.as_ref() {
            Some(key_value) => key_value,
            None => continue,
        };

        match key.as_str() {
            key::JOB_WORKERS => result.job.workers = parse(value, line)?,
            _ => return ris_util::result_err!("unkown key at line {}", i),
        }
    }

    Ok(result)
}

fn compose<T: Display>(value: &Option<T>) -> String {
    match value.as_ref() {
        Some(value) => (*value).to_string(),
        None => String::from(DEFAULT),
    }
}

fn parse<T>(value: &str, line: usize) -> RisResult<Option<T>>
where
    T: FromStr,
    T::Err: Display,
{
    if value == DEFAULT {
        Ok(None)
    } else {
        match value.parse::<T>() {
            Ok(parsed) => Ok(Some(parsed)),
            Err(error) => error_on_line(line, &error.to_string()),
        }
    }
}

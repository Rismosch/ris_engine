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
    panic!()
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

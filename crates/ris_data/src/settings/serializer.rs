use std::path::PathBuf;

use ris_error::RisResult;
use ris_io::fallback_file::FallbackFileOverwrite;

use crate::info::app_info::AppInfo;
use crate::ris_yaml::RisYaml;
use crate::settings::Settings;

//pub const DEFAULT: &str = "default";
pub const DIRECTORY_NAME: &str = "settings";
pub const EXTENSION: &str = ".ris_yaml";

#[derive(Debug, PartialEq, Eq)]
pub enum SerializeError {
    EntryWasEmpty,
    ParseFailed,
    UnkownKey,
}

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

        //let bytes = write_bytes(settings);
        let mut yaml = RisYaml::default();
        settings.serialize(&mut yaml);

        let string = yaml.serialize()?;
        let bytes = string.as_bytes().to_vec();

        self.fallback_file.overwrite_current(&bytes)?;

        ris_log::debug!("settings serialized!");

        Ok(())
    }

    pub fn deserialize(&self, app_info: &AppInfo) -> Option<Settings> {
        ris_log::debug!("deserializing settings...");

        for available_path in self.fallback_file.available_paths() {
            if let Some(bytes) = self.fallback_file.get_by_path(&available_path) {
                match deserialize(&bytes, app_info) {
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

    pub fn parse<F: std::str::FromStr>(value: &str) -> Result<F, SerializeError> {
        value.parse().map_err(|_| SerializeError::ParseFailed)
    }
}

fn deserialize(bytes: &[u8], app_info: &AppInfo) -> RisResult<Settings> {
    let string = String::from_utf8(bytes.to_vec())?;

    let mut settings = Settings::new(app_info);
    let yaml = RisYaml::deserialize(string)?;

    for (i, entry) in yaml.entries.iter().enumerate() {
        match settings.deserialize(entry) {
            Ok(()) => (),
            Err(SerializeError::EntryWasEmpty) => (),
            Err(SerializeError::ParseFailed) => {
                return ris_error::new_result!(
                    "cannot parse value at line {}: {}",
                    i,
                    entry.raw_line
                );
            }
            Err(SerializeError::UnkownKey) => {
                return ris_error::new_result!("unkown key at line {}: {}", i, entry.raw_line);
            }
        }
    }

    Ok(settings)
}

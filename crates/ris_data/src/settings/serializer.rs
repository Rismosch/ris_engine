use std::io::Read;
use std::io::Write;
use std::io::Seek;

use ris_util::error::RisResult;

use crate::settings::Settings;

pub const SETTINGS_DIRECTORY_NAME: &str = "settings";
pub const SETTINGS_FILE_NAME: &str = "ris_settings";

pub fn serialize(settings: &Settings, out_stream: &mut (impl Write + Seek)) -> RisResult<()> {
    panic!("not implemented")
}

pub fn deserialize(in_stream: &mut (impl Read + Seek)) -> Option<Settings> {
    panic!("not implemented")
}

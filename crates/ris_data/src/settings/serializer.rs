use std::io::Read;
use std::io::Write;
use std::io::Seek;

use ris_util::error::RisResult;

use crate::settings::Settings;

pub fn serialize(settings: &Settings, out_stream: &mut (impl Write + Seek)) -> RisResult<()> {
    panic!("not implemented")
}

pub fn deserialize(in_stream: &mut (impl Read + Seek)) -> RisResult<Settings> {
    panic!("not implemented")
}

use std::io::Read;
use std::io::Seek;

use ris_util::ris_error::RisError;

pub const IN_EXT: &str = "glsl";
pub const OUT_EXT: &str = "spirv";

pub fn import(stream: impl Read + Seek) -> Result<Vec<u8>, RisError> {
    ris_log::debug!("hello from glsl importer");
    let result = Vec::new();

    Ok(result)
}

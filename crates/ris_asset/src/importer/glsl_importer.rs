use std::fs::File;

use ris_util::ris_error::RisError;

pub const IN_EXT: &str = "glsl";
pub const OUT_EXT: &str = "spirv";

pub fn import(info: crate::asset_importer::ImportInfo) -> Result<Vec<u8>, RisError> {
    let result = Vec::new();

    Ok(result)
}

use std::io::Cursor;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;

use ris_util::ris_error::RisError;

use crate::AssetId;
use crate::RisAssetData;
use crate::loader::ris_loader;

pub struct Scenes {
    vertex_shader: AssetId,
    fragment_shader: AssetId,
}

pub fn load(bytes: &[u8]) -> Result<Scenes, RisError> {
    let cursor = Cursor::new(bytes);
    let test = ris_loader::load(&mut cursor)?;

    let vertex_shader = data.references[0].clone();
    let fragment_shader = data.references[1].clone();
    Ok(Scenes{
        vertex_shader,
        fragment_shader,
    })
}

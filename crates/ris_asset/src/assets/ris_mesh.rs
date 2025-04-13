use std::io::Cursor;

use ris_error::prelude::*;
use ris_io::FatPtr;

use super::ris_header::RisHeader;

// ris_mesh\0\0\0\0\0\0\0\0
pub const MAGIC: [u8; 16] = [
    0x72, 0x69, 0x73, 0x5f, 0x6D, 0x65, 0x73, 0x68, 0x0A, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];
pub const EXTENSION: &str = "ris_mesh";

pub struct RisMesh {
    pub p_vertices: FatPtr,
    pub p_normals: FatPtr,
    pub p_uvs: FatPtr,
    pub p_indices: FatPtr,
    pub data: Vec<u8>,
}

pub fn serialize(mesh: &RisMesh) -> RisResult<Vec<u8>> {

    panic!()
}

pub fn deserialize(bytes: &[u8]) -> RisResult<RisMesh> {
    let header = RisHeader::deserialize(bytes)?.into_ris_error()?;
    header.assert_magic(MAGIC)?;

    let content = header.content(bytes)?;
    let mut stream = Cursor::new(content);
    let s = &mut stream;

    let p_vertices = ris_io::read_fat_ptr(s)?;
    let p_normals = ris_io::read_fat_ptr(s)?;
    let p_uvs = ris_io::read_fat_ptr(s)?;
    let p_indices = ris_io::read_fat_ptr(s)?;
    let data = ris_io::read_to_end(s)?;

    Ok(RisMesh{
        p_vertices,
        p_normals,
        p_uvs,
        p_indices,
        data,
    })
}

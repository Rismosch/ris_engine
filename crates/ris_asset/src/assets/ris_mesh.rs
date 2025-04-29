use std::io::Cursor;

use ris_asset_data::mesh::CpuMesh;
use ris_error::prelude::*;

use super::ris_header::RisHeader;

// ris_mesh\0\0\0\0\0\0\0\0
pub const MAGIC: [u8; 16] = [
    0x72, 0x69, 0x73, 0x5f, 0x6D, 0x65, 0x73, 0x68, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];
pub const EXTENSION: &str = "ris_mesh";

pub const COMPRESSION_LEVEL: u8 = 6;

pub fn serialize(mesh: &CpuMesh) -> RisResult<Vec<u8>> {
    let mut stream = Cursor::new(Vec::new());
    let s = &mut stream;

    ris_io::write_fat_ptr(s, mesh.p_vertices)?;
    ris_io::write_fat_ptr(s, mesh.p_normals)?;
    ris_io::write_fat_ptr(s, mesh.p_uvs)?;
    ris_io::write_fat_ptr(s, mesh.p_indices)?;
    ris_io::write(s, &mesh.data)?;

    let bytes = stream.into_inner();
    let compressed = miniz_oxide::deflate::compress_to_vec(&bytes, COMPRESSION_LEVEL);

    ris_log::trace!(
        "compressed {} to {}. percentage: {}",
        bytes.len(),
        compressed.len(),
        compressed.len() as f32 / bytes.len() as f32,
    );

    let header = RisHeader::new(MAGIC, Vec::new());
    header.serialize(&compressed)
}

pub fn deserialize(bytes: &[u8]) -> RisResult<CpuMesh> {
    let (header, content) = RisHeader::deserialize(bytes)?.into_ris_error()?;
    header.assert_magic(MAGIC)?;

    let uncompressed = miniz_oxide::inflate::decompress_to_vec(content)
        .map_err(|e| ris_error::new!("failed to decompress: {:?}", e))?;

    let mut stream = Cursor::new(uncompressed);
    let s = &mut stream;

    let p_vertices = ris_io::read_fat_ptr(s)?;
    let p_normals = ris_io::read_fat_ptr(s)?;
    let p_uvs = ris_io::read_fat_ptr(s)?;
    let p_indices = ris_io::read_fat_ptr(s)?;
    let data = ris_io::read_to_end(s)?;

    Ok(CpuMesh {
        p_vertices,
        p_normals,
        p_uvs,
        p_indices,
        data,
    })
}

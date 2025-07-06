use std::io::Cursor;

use ash::vk;

use ris_asset_data::terrain_mesh::TerrainCpuMesh;
use ris_error::prelude::*;

use super::ris_header::RisHeader;

// ris_terrain\0\0\0\0\0
pub const MAGIC: [u8; 16] = [0x72,0x69,0x73,0x5F,0x74,0x65,0x72,0x72,0x61,0x69,0x6E,0x00,0x00,0x00,0x00,0x00];
pub const EXTENSION: &str = "ris_terrain";
pub const COMPRESSION_LEVEL: u8 = 6;

pub fn serialize(mesh: &TerrainCpuMesh) -> RisResult<Vec<u8>> {
    let mut stream = Cursor::new(Vec::new());
    let s = &mut stream;

    ris_io::write_fat_ptr(s, mesh.p_vertices)?;
    ris_io::write_fat_ptr(s, mesh.p_indices)?;
    ris_io::write_i32(s, mesh.index_type.as_raw())?;

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

pub fn deserialize(bytes: &[u8]) -> RisResult<TerrainCpuMesh> {
    let (header, content) = RisHeader::deserialize(bytes)?.into_ris_error()?;
    header.assert_magic(MAGIC)?;

    let decompressed = miniz_oxide::inflate::decompress_to_vec(&content)
        .map_err(|e| ris_error::new!("failed to decompress: {:?}", e))?;

    let mut stream = Cursor::new(decompressed);
    let s = &mut stream;

    let p_vertices = ris_io::read_fat_ptr(s)?;
    let p_indices = ris_io::read_fat_ptr(s)?;
    let index_type = vk::IndexType::from_raw(ris_io::read_i32(s)?);
    let data = ris_io::read_to_end(s)?;

    Ok(TerrainCpuMesh{
        p_vertices,
        p_indices,
        index_type,
        data,
    })
}

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
    ris_error::new_result!("todo")
}

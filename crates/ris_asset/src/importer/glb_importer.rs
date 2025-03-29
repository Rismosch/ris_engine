use std::path::Path;
use std::io::SeekFrom;

use ris_error::prelude::*;

use crate::codecs::gltf::Gltf;

pub const IN_EXT_GLB: &str = "glb";

#[derive(Debug, PartialEq, Eq)]
enum ChunkType {
    Json,
    Bin,
}

struct Chunk {
    chunk_type: ChunkType,
    data: Vec<u8>,
}

pub fn import(source: impl AsRef<Path>, target_dir: impl AsRef<Path>) -> RisResult<()> {
    let source = source.as_ref();
    let target_dir = target_dir.as_ref();

    let mut file = std::fs::File::open(source)?;
    let f = &mut file;

    // header
    let magic = ris_io::read_u32(f)?;
    let version = ris_io::read_u32(f)?;
    let length = ris_io::read_u32(f)?;

    ris_error::assert!(magic == 0x46546C67)?;
    ris_error::assert!(version == 2)?;

    let current = ris_io::seek(f, SeekFrom::Current(0))?;
    let file_length = ris_io::seek(f, SeekFrom::End(0))?;
    ris_io::seek(f, SeekFrom::Start(current))?;

    ris_error::assert!(file_length == length as u64)?;

    // read chunks
    let mut chunks = Vec::new();
    loop {
        let current = ris_io::seek(f, SeekFrom::Current(0))?;
        ris_error::assert!(current % 4 == 0)?;
        if current == file_length {
            break;
        }

        let chunk_length = ris_io::read_u32(f)?;
        let chunk_type = ris_io::read_u32(f)?;

        let chunk_type = match chunk_type {
            0x4E4F534A => ChunkType::Json,
            0x004E4942 => ChunkType::Bin,
            _ => {
                ris_io::seek(f, SeekFrom::Current(chunk_length.into()))?;
                continue;
            }
        };

        let mut chunk = Chunk {
            chunk_type,
            data: vec![0; chunk_length as usize],
        };

        ris_io::read(f, &mut chunk.data)?;
        chunks.push(chunk);
    }

    // identify chunks
    ris_error::assert!(chunks.len() == 2)?;

    let mut chunks = chunks.into_iter();
    let json_chunk = chunks.next().into_ris_error()?;
    let bin_chunk = chunks.next().into_ris_error()?;
    ris_error::assert!(json_chunk.chunk_type == ChunkType::Json)?;
    ris_error::assert!(bin_chunk.chunk_type == ChunkType::Bin)?;

    // import gltf
    let json = String::from_utf8(json_chunk.data)?;
    let gltf = Gltf::deserialize(json)?;

    //ris_log::error!("gltf: {:#?}", gltf);
    for item in gltf.skins.iter() {
        ris_log::error!("item: {:?}", item);
    }
    
    // convert to ris assets
    ris_error::new_result!("not implemented")
}

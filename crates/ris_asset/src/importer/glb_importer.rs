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
    let bin = bin_chunk.data;
    let gltf = Gltf::deserialize(json)?;

    // a glb file may have only a single gltf buffer. the first buffer
    // MUST have it's uri undefined. this way, the buffer references the
    // bin chunk of the glb file
    ris_error::assert!(gltf.buffers.len() == 1)?;
    ris_error::assert!(gltf.buffers[0].uri.is_none())?;

    // convert to internal format. 
    // note that this importer makes assumptions about the underlying
    // data. thus it may return errors on valid gltf

    // meshes
    for mesh in gltf.meshes.iter() {
        ris_log::fatal!("mesh: {:#?}", mesh);

        for primitive in mesh.primitives.iter() {
            for attribute in primitive.attributes.iter() {
                let data = access_data(
                    &bin,
                    &gltf,
                    attribute.accessor,
                )?;

                ris_log::fatal!("data: {:?}", data.len());
            }
        }
    }

    Ok(())
}

fn access_data<'a>(
    bin: &'a [u8],
    gltf: &'a Gltf,
    accessor_index: usize,
) -> RisResult<&'a [u8]> {
    let accessor = gltf.accessors.get(accessor_index).into_ris_error()?;

    let buffer_view_index = accessor.buffer_view.into_ris_error()?;
    let buffer_view = gltf.buffer_views.get(buffer_view_index).into_ris_error()?;
    ris_error::assert!(buffer_view.buffer == 0)?;
    ris_error::assert!(buffer_view.byte_stride.is_none())?;

    let element_size = accessor.component_type.size_in_bytes() * accessor.accessor_type.number_of_components();
    
    let start = accessor.byte_offset + buffer_view.byte_offset;
    let len = accessor.count * element_size;
    let end = start + len;

    ris_error::assert!(len <= buffer_view.byte_length)?;
    ris_error::assert!(start <= end)?;
    ris_error::assert!(end <= bin.len())?;

    Ok(&bin[start..end])
}

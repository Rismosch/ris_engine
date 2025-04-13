use std::path::Path;
use std::io::SeekFrom;
use std::process::exit;

use ris_error::prelude::*;

use crate::codecs::gltf::Accessor;
use crate::codecs::gltf::AccessorComponentType;
use crate::codecs::gltf::AccessorType;
use crate::codecs::gltf::Gltf;
use crate::codecs::gltf::MeshPrimitiveAttributeName;
use crate::codecs::gltf::MeshPrimitiveMode;

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

    let source_file_stem = source.file_stem()
        .into_ris_error()?
        .to_str()
        .into_ris_error()?;

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
    for (mesh_index, mesh) in gltf.meshes.iter().enumerate() {
        ris_log::fatal!("mesh: {:#?}", mesh);
        for (primitive_index, primitive) in mesh.primitives.iter().enumerate() {
            ris_error::assert!(primitive.mode == MeshPrimitiveMode::Triangles)?;

            let vertex_attribute = primitive.get_attribute(MeshPrimitiveAttributeName::Position)
                .into_ris_error()?;
            let normal_attribute = primitive.get_attribute(MeshPrimitiveAttributeName::Normal)
                .into_ris_error()?;
            let uv_attribute = primitive.get_attribute(MeshPrimitiveAttributeName::TexCoord(0))
                .into_ris_error()?;
            let index_accessor_index = primitive.indices.into_ris_error()?;

            let vertex_accessor = gltf.accessors.get(vertex_attribute.accessor).into_ris_error()?;
            let normal_accessor = gltf.accessors.get(normal_attribute.accessor).into_ris_error()?;
            let uv_accessor = gltf.accessors.get(uv_attribute.accessor).into_ris_error()?;
            let index_accessor = gltf.accessors.get(index_accessor_index).into_ris_error()?;

            ris_error::assert!(vertex_accessor.count == normal_accessor.count)?;
            ris_error::assert!(vertex_accessor.count == uv_accessor.count)?;
            ris_error::assert!(vertex_accessor.accessor_type == AccessorType::Vec3)?;
            ris_error::assert!(vertex_accessor.component_type == AccessorComponentType::F32)?;
            ris_error::assert!(normal_accessor.accessor_type == AccessorType::Vec3)?;
            ris_error::assert!(normal_accessor.component_type == AccessorComponentType::F32)?;
            ris_error::assert!(uv_accessor.accessor_type == AccessorType::Vec2)?;
            ris_error::assert!(uv_accessor.component_type == AccessorComponentType::F32)?;
            ris_error::assert!(index_accessor.accessor_type == AccessorType::Scalar)?;
            ris_error::assert!(index_accessor.component_type == AccessorComponentType::U16)?;

            let vertex_data = access_data(vertex_accessor, &bin, &gltf)?;
            let normal_data = access_data(normal_accessor, &bin, &gltf)?;
            let uv_data = access_data(uv_accessor, &bin, &gltf)?;
            let index_data = access_data(index_accessor, &bin, &gltf)?;

            ris_log::fatal!("vertices: {:?}", vertex_data.len());
            ris_log::fatal!("normals: {:?}", normal_data.len());
            ris_log::fatal!("uvs: {:?}", uv_data.len());
            ris_log::fatal!("indices: {:?}", index_data.len());

            let mesh_name = if let Some(name) = &mesh.name {
                name.clone()
            } else {
                "none".to_string()
            };

            let target_name = format!(
                "{}-{}-{:03}-{:03}.{}",
                source_file_stem,
                mesh_name,
                mesh_index,
                primitive_index,
                crate::assets::ris_mesh::EXTENSION,
            );
            let target_path = target_dir.join(&target_name);

            ris_log::fatal!("target path: {:?}", target_path);
        }
    }

    Ok(())
}

fn access_data<'a>(
    accessor: &Accessor,
    bin: &'a [u8],
    gltf: &'a Gltf,
) -> RisResult<&'a [u8]> {
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

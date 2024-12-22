use std::io::Cursor;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::Read;
use std::io::Write;

use ris_data::ecs::decl::GameObjectHandle;
use ris_data::ecs::handle::GenericHandle;
use ris_data::ecs::scene::Scene;
use ris_error::Extensions;
use ris_error::RisResult;
use ris_io::FatPtr;

use super::ris_header::RisHeader;

// ris_scene\0\0\0\0\0\0\0
pub const MAGIC: [u8; 16] = [0x72,0x69,0x73,0x5f,0x73,0x63,0x65,0x6e,0x65,0x00,0x00,0x00,0x00,0x00,0x00,0x00]; 
pub const EXTENSION: &str = "ris_scene";

pub struct SceneStream {
    inner: Cursor<Vec<u8>>,
}

#[derive(Debug)]
struct Placeholder {
    id: usize,
    references: Vec<PlaceholderReference>,
}

#[derive(Debug)]
struct PlaceholderReference {
    id: usize,
    fat_ptr: FatPtr,
}

impl Seek for SceneStream {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        self.inner.seek(pos)
    }
}

impl Read for SceneStream {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.inner.read(buf)
    }
}

impl Write for SceneStream {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.inner.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.inner.flush()
    }
}

pub fn serialize(scene: &Scene, chunk_index: usize) -> RisResult<Vec<u8>> {
    ris_error::debug_assert!(chunk_index < scene.static_chunks.len())?;
    let chunk = &scene.static_chunks[chunk_index];

    let handles = chunk
        .game_objects
        .iter()
        .filter(|x| x.borrow().is_alive)
        .map(|x| x.borrow().handle)
        .collect::<Vec<_>>();

    let mut stream = SceneStream{
        inner: Cursor::new(Vec::new()),
    };
    let f = &mut stream;

    let mut placeholders = Vec::new();

    // serialize game objects
    ris_io::write_uint(f, handles.len())?;
    for (i, generic_handle) in handles.into_iter().enumerate() {
        let mut placeholder = Placeholder {
            id: generic_handle.scene_id().index,
            references: Vec::new(),
        };

        let handle: GameObjectHandle = generic_handle.into();

        ris_io::write_string(f, handle.name(scene)?)?;
        ris_io::write_bool(f, handle.is_active(scene)?)?;
        ris_io::write_vec3(f, handle.local_position(scene)?)?;
        ris_io::write_quat(f, handle.local_rotation(scene)?)?;
        ris_io::write_f32(f, handle.local_scale(scene)?)?;
         
        // todo: components

        match handle.parent(scene)? {
            Some(parent_handle) => {
                let id = parent_handle.0.scene_id().index;
                let fat_ptr = ris_io::write_uint(f, id)?;
                placeholder.references.push(PlaceholderReference{
                    id,
                    fat_ptr,
                });
            },
            None => {
                // its own id (i) represents no parent
                ris_io::write_uint(f, i)?;
            },
        };

        //let children = handle.children(scene)?;
        //let child_count = children.len();
        //ris_io::write_uint(f, child_count)?;
        //for child in children {
        //    let id = child.0.scene_id().index;
        //    let fat_ptr = ris_io::write_uint(f, id)?;
        //    placeholder.references.push(PlaceholderReference{
        //        id,
        //        fat_ptr,
        //    });
        //}

        // add to lookup
        placeholders.push(placeholder);
    } // serialize game objects END
    
    // resolve placeholder references
    for placeholder in placeholders.iter() {
        for reference in placeholder.references.iter() {
            let actual_id = placeholders
                .iter()
                .position(|x| reference.id == x.id)
                .into_ris_error()
                .map_err(|e| {
                    ris_log::error!("failed to find reference. make sure that a scene only references game objects in it's own chunk");
                    e
                })?;

            ris_io::seek(f, SeekFrom::Start(reference.fat_ptr.addr))?;
            ris_io::write_uint(f, actual_id)?;
        }
    }


    // compress
    let bytes = stream.inner.into_inner();

    let compressed = miniz_oxide::deflate::compress_to_vec(&bytes, 6);
    ris_log::trace!(
        "compressed {} to {}. percentage: {}",
        bytes.len(),
        compressed.len(),
        compressed.len() as f32 / bytes.len() as f32,
    );

    // add header
    let mut stream = Cursor::new(Vec::new());
    let f = &mut stream;

    let header = RisHeader::new(MAGIC, Vec::new());
    let header_bytes = header.serialize()?;
    ris_io::write(f, &header_bytes)?;
    ris_io::write(f, &compressed)?;

    let result = stream.into_inner();
    Ok(result)
}

pub fn load(scene: &Scene, bytes: &[u8]) -> RisResult<Option<usize>> {
    let reserved = scene.reserve_chunk();
    let Some(index) = reserved else {
        return Ok(None);
    };

    let chunk = &scene.static_chunks[index];

    let header = RisHeader::load(bytes)?.into_ris_error()?;
    header.assert_magic(MAGIC)?;

    let content = header.content(bytes)?;
    let uncompressed = miniz_oxide::inflate::decompress_to_vec(content).map_err(|e| {
        ris_error::new!("failed to decompress: {:?}", e)
    })?;

    let mut stream = Cursor::new(uncompressed);
    let f = &mut stream;

    let game_object_count = ris_io::read_uint(f)?;
    let mut lookup = Vec::with_capacity(game_object_count);
    let mut parents_to_assign = Vec::with_capacity(game_object_count);

    // deserialize game objects
    for i in 0..game_object_count {
        let name = ris_io::read_string(f)?;
        let is_active = ris_io::read_bool(f)?;
        let local_position = ris_io::read_vec3(f)?;
        let local_rotation = ris_io::read_quat(f)?;
        let local_scale = ris_io::read_f32(f)?;
        let parent_id = ris_io::read_uint(f)?;

        let game_object = GameObjectHandle::new_static(scene, index)?;
        let id = game_object.0.scene_id().index;
        lookup.push(id);

        game_object.set_name(scene, &name)?;
        game_object.set_active(scene, is_active)?;
        game_object.set_local_position(scene, local_position)?;
        game_object.set_local_rotation(scene, local_rotation)?;
        game_object.set_local_scale(scene, local_scale)?;

        if i != parent_id {
            let parent_to_assign = (game_object, parent_id);
            parents_to_assign.push(parent_to_assign);
        }
    } // deserialize game objects End

    // assign parents
    for (game_object, parent_id) in parents_to_assign {
        let actual_parent_id = *lookup.get(parent_id).into_ris_error()?;
        let parent: GameObjectHandle = chunk.game_objects.iter()
            .find(|x| x.borrow().handle.scene_id().index == actual_parent_id)
            .into_ris_error()?
            .borrow()
            .handle
            .into();

        game_object.set_parent(scene, Some(parent), 0, false)?;
    }

    Ok(Some(index))
}


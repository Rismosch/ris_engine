use std::io::Cursor;
use std::io::SeekFrom;

use ris_data::ecs::decl::GameObjectHandle;
use ris_data::ecs::scene::Scene;
use ris_data::ecs::scene_stream::SceneReader;
use ris_data::ecs::scene_stream::SceneWriter;
use ris_error::Extensions;
use ris_error::RisResult;
use ris_io::FatPtr;

use super::ris_header::RisHeader;

// ris_scene\0\0\0\0\0\0\0
pub const MAGIC: [u8; 16] = [0x72,0x69,0x73,0x5f,0x73,0x63,0x65,0x6e,0x65,0x00,0x00,0x00,0x00,0x00,0x00,0x00];
pub const EXTENSION: &str = "ris_scene";

pub const COMPRESSION_LEVEL: u8 = 6;

pub fn serialize(scene: &Scene, chunk_index: usize) -> RisResult<Vec<u8>> {
    ris_error::debug_assert!(chunk_index < scene.static_chunks.len())?;
    let chunk = &scene.static_chunks[chunk_index];

    let handles = chunk
        .game_objects
        .iter()
        .filter(|x| x.borrow().is_alive)
        .map(|x| x.borrow().handle)
        .collect::<Vec<_>>();

    let mut stream = SceneWriter::new(chunk_index, scene);
    let f = &mut stream;

    let mut lookup = Vec::new();

    // serialize game objects
    ris_io::write_uint(f, handles.len())?;
    for generic_handle in handles.into_iter() {
        let handle: GameObjectHandle = generic_handle.into();
        let scene_index = handle.0.scene_id().index;
        lookup.push(scene_index);

        ris_io::write_string(f, handle.name(scene)?)?;
        ris_io::write_bool(f, handle.is_active(scene)?)?;
        ris_io::write_vec3(f, handle.local_position(scene)?)?;
        ris_io::write_quat(f, handle.local_rotation(scene)?)?;
        ris_io::write_f32(f, handle.local_scale(scene)?)?;
         
        let components = handle.components(scene)?;
        ris_io::write_uint(f, components.len())?;
        for component in components {
            let ptr_addr = ris_io::write_fat_ptr(f, FatPtr::null())?.addr; // placeholder ptr
            let addr = ris_io::seek(f, SeekFrom::Current(0))?;

            let position = scene.registry.component_factories()
                .iter()
                .position(|x| x.component_id() == component.type_id())
                .into_ris_error()?;

            ris_io::write_uint(f, position)?;
            scene.deref_mut_component(component, |x| x.serialize(f))??;

            // fill placeholder ptr
            let end = ris_io::seek(f, SeekFrom::Current(0))?;
            let ptr = FatPtr::begin_end(addr, end)?;
            ris_io::seek(f, SeekFrom::Start(ptr_addr))?;
            ris_io::write_fat_ptr(f, ptr)?;
            ris_io::seek(f, SeekFrom::Start(end))?;
        }

        let children = handle.children(scene)?;
        let child_count = children.len();
        ris_io::write_uint(f, child_count)?;
        for child in children {
            f.write_game_object(child)?;
        }
    }
    
    // resolve
    let bytes = stream.resolve(lookup)?;

    // compress
    let compressed = miniz_oxide::deflate::compress_to_vec(&bytes, COMPRESSION_LEVEL);
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

    let mut stream = SceneReader::new(index, scene, uncompressed);
    let f = &mut stream;

    let game_object_count = ris_io::read_uint(f)?;
    
    f.lookup = Vec::with_capacity(game_object_count);
    let mut children_to_assign = Vec::with_capacity(game_object_count);
    let mut components_to_deserialize = Vec::with_capacity(game_object_count);

    // deserialize game objects
    for _ in 0..game_object_count {
        let name = ris_io::read_string(f)?;
        let is_active = ris_io::read_bool(f)?;
        let local_position = ris_io::read_vec3(f)?;
        let local_rotation = ris_io::read_quat(f)?;
        let local_scale = ris_io::read_f32(f)?;

        let component_count = ris_io::read_uint(f)?;
        let mut component_ptrs = Vec::with_capacity(component_count);
        for _ in 0..component_count {
            let ptr = ris_io::read_fat_ptr(f)?;
            component_ptrs.push(ptr);

            ris_io::seek(f, SeekFrom::Current(ptr.len.try_into()?))?;
        }

        let child_count = ris_io::read_uint(f)?;
        let mut child_ids = Vec::with_capacity(child_count);
        for _ in 0..child_count {
            let child_id = ris_io::read_uint(f)?;
            child_ids.push(child_id);
        }

        let game_object = GameObjectHandle::new_static(scene, index)?;
        let id = game_object.0.scene_id().index;
        f.lookup.push(id);

        game_object.set_name(scene, &name)?;
        game_object.set_active(scene, is_active)?;
        game_object.set_local_position(scene, local_position)?;
        game_object.set_local_rotation(scene, local_rotation)?;
        game_object.set_local_scale(scene, local_scale)?;

        children_to_assign.push((game_object, child_ids));
        components_to_deserialize.push((game_object, component_ptrs));
    }

    // assign children
    for (game_object, child_ids) in children_to_assign {
        for (i, &child_id) in child_ids.iter().enumerate() {
            let actual_id = f.lookup.get(child_id).into_ris_error()?;

            let child: GameObjectHandle = chunk.game_objects.iter()
                .find(|x| x.borrow().handle.scene_id().index == *actual_id)
                .into_ris_error()?
                .borrow()
                .handle
                .into();

            child.set_parent(scene, Some(game_object), i, false)?;
        }
    }

    // deserialize components
    for (game_object, component_ptrs) in components_to_deserialize {
        for FatPtr { addr, len: _ } in component_ptrs {
            ris_io::seek(f, SeekFrom::Start(addr))?;

            let position = ris_io::read_uint(f)?;
            let factory = scene.registry
                .component_factories()
                .get(position)
                .into_ris_error()?;

            let component = factory.make(scene, game_object)?;
            scene.deref_mut_component(component, |x| x.deserialize(f))??;
        }
    }

    Ok(Some(index))
}


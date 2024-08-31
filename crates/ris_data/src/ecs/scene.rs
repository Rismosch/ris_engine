use crate::ptr::ArefCell;
use crate::ptr::StrongPtr;
use crate::ptr::WeakPtr;

use super::error::EcsError;
use super::error::EcsResult;
use super::game_object::GameObject;
use super::handle::GenericHandle;
use super::id::Component;
use super::id::EcsInstance;
use super::id::EcsObject;
use super::id::EcsPtr;
use super::id::EcsWeakPtr;
use super::id::SceneKind;
use super::id::SceneId;
use super::mesh_component::MeshComponent;
use super::script_component::ScriptComponent;

const DEFAULT_MOVABLE_GAME_OBJECTS: usize = 1024;
const DEFAULT_STATIC_CHUNKS: usize = 8;
const DEFAULT_STATIC_GAME_OBJECTS_PER_CHUNK: usize = 1024;
const DEFAULT_MESH_COMPONENTS: usize = 1024;
const DEFAULT_SCRIPT_COMPONENTS: usize = 1024;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SceneCreateInfo {
    pub movable_game_objects: usize,
    pub static_chunks: usize,
    pub static_game_objects_per_chunk: usize,
    pub mesh_components: usize,
    pub script_components: usize,
}

pub struct Scene {
    pub movable_game_objects: Vec<EcsPtr<GameObject>>,
    pub static_game_objects: Vec<Vec<EcsPtr<GameObject>>>,
    pub mesh_components: Vec<EcsPtr<MeshComponent>>,
    pub script_components: Vec<EcsPtr<ScriptComponent>>,
}

impl Default for SceneCreateInfo {
    fn default() -> Self {
        Self {
            movable_game_objects: DEFAULT_MOVABLE_GAME_OBJECTS,
            static_chunks: DEFAULT_STATIC_CHUNKS,
            static_game_objects_per_chunk: DEFAULT_STATIC_GAME_OBJECTS_PER_CHUNK,
            mesh_components: DEFAULT_MESH_COMPONENTS,
            script_components: DEFAULT_SCRIPT_COMPONENTS,
        }
    }
}

impl Scene {
    pub fn new(info: SceneCreateInfo) -> EcsResult<Self> {
        let movable_game_objects = create_chunk(SceneKind::MovableGameObject, info.movable_game_objects)?;

        let mut static_game_objects = Vec::with_capacity(info.static_chunks);
        for i in 0..info.static_chunks {
            let kind = SceneKind::StaticGameObjct{chunk: i};
            let chunk = create_chunk(kind,info.static_game_objects_per_chunk)?;
            static_game_objects.push(chunk);
        }

        let mesh_components = create_chunk(SceneKind::Component, info.mesh_components)?;
        let script_components = create_chunk(SceneKind::Component, info.script_components)?;

        Ok(Self {
            movable_game_objects,
            static_game_objects,
            mesh_components,
            script_components,
        })
    }

    pub fn deref<T: EcsObject>(&self, handle: GenericHandle<T>) -> EcsResult<EcsWeakPtr<T>> {
        let chunk = self.find_chunk(handle.scene_id().kind)?;
        let index = handle.scene_id().index;
        let ptr = &chunk[index];
        let aref = ptr.borrow();

        let is_alive = aref.is_alive;
        let generation_matches = aref.handle.generation() == handle.generation();

        if is_alive && generation_matches {
            Ok(ptr.to_weak())
        } else {
            Err(EcsError::ObjectIsDestroyed)
        }
    }

    pub fn create_new<T: EcsObject>(&self, kind: SceneKind) -> EcsResult<EcsWeakPtr<T>> {
        let chunk = self.find_chunk(kind)?;

        let Some(position) = chunk.iter().position(|x| !x.borrow().is_alive) else {
            return Err(EcsError::OutOfMemory);
        };

        let ptr = &chunk[position];
        let old_handle = ptr.borrow().handle;
        let new_generation = old_handle.generation().wrapping_add(1);
        let new_handle = GenericHandle::new(old_handle.scene_id(), new_generation)?;

        let mut aref_mut = ptr.borrow_mut();
        aref_mut.handle = new_handle;
        aref_mut.is_alive = true;
        aref_mut.value = T::default();
        drop(aref_mut);

        Ok(ptr.to_weak())
    }

    pub fn mark_as_destroyed<T: EcsObject>(&self, handle: GenericHandle<T>) {
        let Ok(chunk) = self.find_chunk::<T>(handle.scene_id().kind) else {
            return;
        };
        let index = handle.scene_id().index;
        let ptr = &chunk[index];
        ptr.borrow_mut().is_alive = false;
    }


    fn find_chunk<T: EcsObject>(&self, kind: SceneKind) -> EcsResult<&[EcsPtr<T>]> {
        match kind {
            SceneKind::MovableGameObject => cast(&self.movable_game_objects),
            SceneKind::StaticGameObjct { chunk } => cast(&self.static_game_objects[chunk]),
            SceneKind::Component => match T::ecs_type_id() {
                super::decl::ECS_TYPE_ID_MESH_COMPONENT => cast(&self.mesh_components),
                super::decl::ECS_TYPE_ID_SCRIPT_COMPONENT => cast(&self.script_components),
                _ => return Err(EcsError::OutOfBounds),
            },
        }
    }
}

fn create_chunk<T: EcsObject>(kind: SceneKind, capacity: usize) -> EcsResult<Vec<EcsPtr<T>>> {
    let mut result = Vec::with_capacity(capacity);
    for i in 0..capacity {
        let id = SceneId {
            kind,
            index: i,
        };
        let handle = GenericHandle::new(id, 0)?;
        let instance = EcsInstance::new(handle);
        let ptr = StrongPtr::new(ArefCell::new(instance));
        result.push(ptr);
    }

    Ok(result)
}

fn cast<T: EcsObject, U: EcsObject>(chunk: &[EcsPtr<T>]) -> EcsResult<&[EcsPtr<U>]> { 
    if T::ecs_type_id() != U::ecs_type_id() {
        return Err(EcsError::InvalidCast);
    }

    // transmute is safe, because T is equal to U
    let result = unsafe {std::mem::transmute::<&[EcsPtr<T>], &[EcsPtr<U>]>(chunk)};

    Ok(result)
}


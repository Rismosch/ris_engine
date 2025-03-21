use std::any::TypeId;
use std::sync::Arc;

use ris_ptr::ArefCell;
use ris_ptr::StrongPtr;

use super::components::mesh_renderer::MeshRendererComponent;
use super::components::script::DynScriptComponent;
use super::decl::GameObjectHandle;
use super::error::EcsError;
use super::error::EcsResult;
use super::game_object::GameObject;
use super::handle::DynComponentHandle;
use super::handle::DynHandle;
use super::handle::GenericHandle;
use super::id::Component;
use super::id::EcsInstance;
use super::id::EcsObject;
use super::id::EcsPtr;
use super::id::EcsWeakPtr;
use super::id::SceneId;
use super::id::SceneKind;
use super::mesh::VideoMesh;
use super::registry::Registry;

const DEFAULT_DYNAMIC_GAME_OBJECTS: usize = 1024;
const DEFAULT_STATIC_CHUNKS: usize = 8;
const DEFAULT_GAME_OBJECTS_PER_STATIC_CHUNK: usize = 1024;
const DEFAULT_MESH_RENDERER_COMPONENTS: usize = 1024;
const DEFAULT_SCRIPT_COMPONENTS: usize = 1024;
const DEFAULT_VIDEO_MESHES: usize = 1024;

#[derive(Debug)]
pub struct SceneCreateInfo {
    // game objects
    pub dynamic_game_objects: usize,
    pub static_chunks: usize,
    pub game_objects_per_static_chunk: usize,

    // components
    pub mesh_renderer_components: usize,
    pub script_components: usize,

    // other
    pub video_meshes: usize,
    pub registry: Option<Arc<Registry>>,
}

pub struct StaticChunk {
    is_reserved: ArefCell<bool>,
    pub game_objects: Vec<EcsPtr<GameObject>>,
}

pub struct Scene {
    // game objects
    pub dynamic_game_objects: Vec<EcsPtr<GameObject>>,
    pub static_chunks: Vec<StaticChunk>,

    // compontents
    pub mesh_renderer_components: Vec<EcsPtr<MeshRendererComponent>>,
    pub script_components: Vec<EcsPtr<DynScriptComponent>>,

    // other
    pub video_meshes: Vec<EcsPtr<VideoMesh>>,
    pub registry: Arc<Registry>,
}

impl Default for SceneCreateInfo {
    fn default() -> Self {
        Self {
            dynamic_game_objects: DEFAULT_DYNAMIC_GAME_OBJECTS,
            static_chunks: DEFAULT_STATIC_CHUNKS,
            game_objects_per_static_chunk: DEFAULT_GAME_OBJECTS_PER_STATIC_CHUNK,
            mesh_renderer_components: DEFAULT_MESH_RENDERER_COMPONENTS,
            script_components: DEFAULT_SCRIPT_COMPONENTS,
            video_meshes: DEFAULT_VIDEO_MESHES,
            registry: None,
        }
    }
}

impl SceneCreateInfo {
    pub fn empty() -> Self {
        Self {
            dynamic_game_objects: 0,
            static_chunks: 0,
            game_objects_per_static_chunk: 0,
            mesh_renderer_components: 0,
            script_components: 0,
            video_meshes: 0,
            registry: None,
        }
    }

    pub fn with_single_static_chunk(registry: Arc<Registry>) -> Self {
        Self {
            dynamic_game_objects: 0,
            static_chunks: 1,
            game_objects_per_static_chunk: DEFAULT_GAME_OBJECTS_PER_STATIC_CHUNK,
            mesh_renderer_components: DEFAULT_MESH_RENDERER_COMPONENTS,
            script_components: DEFAULT_SCRIPT_COMPONENTS,
            video_meshes: DEFAULT_VIDEO_MESHES,
            registry: Some(registry),
        }
    }
}

impl Scene {
    pub fn free(&self, device: &ash::Device) {
        for video_mesh in self.video_meshes.iter() {
            let mut aref_mut = video_mesh.borrow_mut();
            aref_mut.free(device);
        }
    }

    pub fn new(info: SceneCreateInfo) -> EcsResult<Self> {
        let Some(registry) = info.registry else {
            return Err(EcsError::InvalidOperation("registry was none".to_string()));
        };

        let dynamic_game_objects =
            create_chunk(SceneKind::DynamicGameObject, info.dynamic_game_objects)?;

        let mut static_chunks = Vec::with_capacity(info.static_chunks);
        for i in 0..info.static_chunks {
            let kind = SceneKind::StaticGameObjct { chunk: i };
            let game_objects = create_chunk(kind, info.game_objects_per_static_chunk)?;
            let chunk = StaticChunk {
                is_reserved: ArefCell::new(false),
                game_objects,
            };
            static_chunks.push(chunk);
        }

        let mesh_renderer_components =
            create_chunk(SceneKind::Component, info.mesh_renderer_components)?;
        let script_components = create_chunk(SceneKind::Component, info.script_components)?;

        let video_meshes = create_chunk(SceneKind::Other, info.video_meshes)?;

        Ok(Self {
            dynamic_game_objects,
            static_chunks,
            mesh_renderer_components,
            script_components,
            video_meshes,
            registry,
        })
    }

    pub fn reserve_chunk(&self) -> Option<usize> {
        let position = self
            .static_chunks
            .iter()
            .position(|x| !*x.is_reserved.borrow());

        if let Some(index) = position {
            let chunk = &self.static_chunks[index];
            *chunk.is_reserved.borrow_mut() = true;
        }

        position
    }

    pub fn clear_chunk(&self, index: usize) {
        ris_error::throw_debug_assert!(index < self.static_chunks.len(), "index was out of bounds",);
        let chunk = &self.static_chunks[index];
        if !*chunk.is_reserved.borrow() {
            ris_log::info!(
                "chunk {} was already cleared and isn't reserved anymore",
                index
            );
            return;
        }

        for ptr in chunk.game_objects.iter() {
            let generic_handle = ptr.borrow().handle;
            let game_object_handle = GameObjectHandle::from(generic_handle);
            game_object_handle.destroy(self);
        }

        *chunk.is_reserved.borrow_mut() = false;
    }

    pub fn deref<T: EcsObject + 'static>(
        &self,
        handle: GenericHandle<T>,
    ) -> EcsResult<EcsWeakPtr<T>> {
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

    pub fn create_new<T: EcsObject + Default + 'static>(
        &self,
        kind: SceneKind,
    ) -> EcsResult<EcsWeakPtr<T>> {
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

    pub fn mark_as_destroyed(&self, handle: DynHandle) -> EcsResult<()> {
        let SceneId { kind, index } = handle.scene_id();
        let type_id = handle.type_id();

        if type_id == TypeId::of::<GameObject>() {
            let chunk = self.find_chunk::<GameObject>(kind)?;
            chunk[index].borrow_mut().is_alive = false;
        } else if type_id == TypeId::of::<MeshRendererComponent>() {
            let chunk = self.find_chunk::<MeshRendererComponent>(kind)?;
            chunk[index].borrow_mut().is_alive = false;
        } else if type_id == TypeId::of::<DynScriptComponent>() {
            let chunk = self.find_chunk::<DynScriptComponent>(kind)?;
            chunk[index].borrow_mut().is_alive = false;
        } else if type_id == TypeId::of::<VideoMesh>() {
            let chunk = self.find_chunk::<VideoMesh>(kind)?;
            chunk[index].borrow_mut().is_alive = false;
        } else {
            return Err(EcsError::InvalidCast);
        }

        Ok(())
    }

    pub fn deref_component<T>(
        &self,
        handle: DynComponentHandle,
        callback: impl FnOnce(&dyn Component) -> T,
    ) -> EcsResult<T> {
        let SceneId { kind, index } = handle.scene_id();
        let type_id = handle.type_id();

        if kind != SceneKind::Component {
            return Err(EcsError::InvalidCast);
        }

        let retval = if type_id == TypeId::of::<MeshRendererComponent>() {
            let aref = self.mesh_renderer_components[index].borrow();
            callback(&aref.value)
        } else if type_id == TypeId::of::<DynScriptComponent>() {
            let aref = self.script_components[index].borrow();
            callback(&aref.value)
        } else {
            return Err(EcsError::InvalidCast);
        };

        Ok(retval)
    }

    pub fn deref_mut_component<T>(
        &self,
        handle: DynComponentHandle,
        callback: impl FnOnce(&mut dyn Component) -> T,
    ) -> EcsResult<T> {
        let SceneId { kind, index } = handle.scene_id();
        let type_id = handle.type_id();

        if kind != SceneKind::Component {
            return Err(EcsError::InvalidCast);
        }

        let retval = if type_id == TypeId::of::<MeshRendererComponent>() {
            let mut aref = self.mesh_renderer_components[index].borrow_mut();
            callback(&mut aref.value)
        } else if type_id == TypeId::of::<DynScriptComponent>() {
            let mut aref = self.script_components[index].borrow_mut();
            callback(&mut aref.value)
        } else {
            return Err(EcsError::InvalidCast);
        };

        Ok(retval)
    }

    fn find_chunk<T: EcsObject + 'static>(&self, kind: SceneKind) -> EcsResult<&[EcsPtr<T>]> {
        match kind {
            SceneKind::Null => Err(EcsError::IsNull),
            SceneKind::DynamicGameObject => cast_chunk(&self.dynamic_game_objects),
            SceneKind::StaticGameObjct { chunk } => {
                cast_chunk(&self.static_chunks[chunk].game_objects)
            }
            SceneKind::Component => {
                let type_id = TypeId::of::<T>();
                if type_id == TypeId::of::<MeshRendererComponent>() {
                    cast_chunk(&self.mesh_renderer_components)
                } else if type_id == TypeId::of::<DynScriptComponent>() {
                    cast_chunk(&self.script_components)
                } else {
                    Err(EcsError::TypeDoesNotMatchSceneKind)
                }
            }
            SceneKind::Other => {
                let type_id = TypeId::of::<T>();
                if type_id == TypeId::of::<VideoMesh>() {
                    cast_chunk(&self.video_meshes)
                } else {
                    Err(EcsError::TypeDoesNotMatchSceneKind)
                }
            }
        }
    }
}

fn create_chunk<T: EcsObject + Default + 'static>(
    kind: SceneKind,
    capacity: usize,
) -> EcsResult<Vec<EcsPtr<T>>> {
    let mut result = Vec::with_capacity(capacity);
    for i in 0..capacity {
        let id = SceneId { kind, index: i };
        let handle = GenericHandle::new(id, 0)?;
        let instance = EcsInstance::new(handle);
        let ptr = StrongPtr::new(ArefCell::new(instance));
        result.push(ptr);
    }

    Ok(result)
}

fn cast_chunk<T: EcsObject + 'static, U: EcsObject + 'static>(
    chunk: &[EcsPtr<T>],
) -> EcsResult<&[EcsPtr<U>]> {
    if TypeId::of::<T>() != TypeId::of::<U>() {
        return Err(EcsError::InvalidCast);
    }

    // transmute is safe, because T is equal to U
    let result = unsafe { std::mem::transmute::<&[EcsPtr<T>], &[EcsPtr<U>]>(chunk) };

    Ok(result)
}

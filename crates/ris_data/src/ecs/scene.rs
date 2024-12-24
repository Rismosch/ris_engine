use ris_ptr::ArefCell;
use ris_ptr::StrongPtr;

use super::components::mesh_renderer::MeshRendererComponent;
use super::components::script::DynScriptComponent;
use super::decl::EcsTypeId;
use super::decl::GameObjectHandle;
use super::error::EcsError;
use super::error::EcsResult;
use super::game_object::GameObject;
use super::handle::DynComponentHandle;
use super::handle::GenericHandle;
use super::id::Component;
use super::id::EcsInstance;
use super::id::EcsObject;
use super::id::EcsPtr;
use super::id::EcsWeakPtr;
use super::id::SceneId;
use super::id::SceneKind;
use super::mesh::VideoMesh;

const DEFAULT_DYNAMIC_GAME_OBJECTS: usize = 1024;
const DEFAULT_STATIC_CHUNKS: usize = 8;
const DEFAULT_STATIC_GAME_OBJECTS_PER_CHUNK: usize = 1024;
const DEFAULT_MESH_RENDERER_COMPONENTS: usize = 1024;
const DEFAULT_SCRIPT_COMPONENTS: usize = 1024;
const DEFAULT_VIDEO_MESHES: usize = 1024;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SceneCreateInfo {
    // game objects
    pub dynamic_game_objects: usize,
    pub static_chunks: usize,
    pub static_game_objects_per_chunk: usize,

    // components
    pub mesh_renderer_components: usize,
    pub script_components: usize,

    // other
    pub video_meshes: usize,
}

pub struct StaticChunk {
    is_reserved: ArefCell<bool>,
    pub game_objects: Vec<EcsPtr<GameObject>>,
}

pub struct Scene {
    pub dynamic_game_objects: Vec<EcsPtr<GameObject>>,
    pub static_chunks: Vec<StaticChunk>,
    pub mesh_renderer_components: Vec<EcsPtr<MeshRendererComponent>>,
    pub script_components: Vec<EcsPtr<DynScriptComponent>>,
    pub video_meshes: Vec<EcsPtr<VideoMesh>>,
}

impl Default for SceneCreateInfo {
    fn default() -> Self {
        Self {
            dynamic_game_objects: DEFAULT_DYNAMIC_GAME_OBJECTS,
            static_chunks: DEFAULT_STATIC_CHUNKS,
            static_game_objects_per_chunk: DEFAULT_STATIC_GAME_OBJECTS_PER_CHUNK,
            mesh_renderer_components: DEFAULT_MESH_RENDERER_COMPONENTS,
            script_components: DEFAULT_SCRIPT_COMPONENTS,
            video_meshes: DEFAULT_VIDEO_MESHES,
        }
    }
}

impl SceneCreateInfo {
    pub fn empty() -> Self {
        Self {
            dynamic_game_objects: 0,
            static_chunks: 0,
            static_game_objects_per_chunk: 0,
            mesh_renderer_components: 0,
            script_components: 0,
            video_meshes: 0,
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
        ris_error::throw_debug_assert!(
            index < self.static_chunks.len(),
            "index was out of bounds",
        );
        let chunk = &self.static_chunks[index];
        if !*chunk.is_reserved.borrow() {
            ris_log::info!("chunk {} was already cleared and isn't reserved anymore", index);
            return;
        }

        for ptr in chunk.game_objects.iter() {
            let generic_handle = ptr.borrow().handle;
            let game_object_handle = GameObjectHandle::from(generic_handle);
            game_object_handle.destroy(self);
        }

        *chunk.is_reserved.borrow_mut() = false;
    }

    pub fn new(info: SceneCreateInfo) -> EcsResult<Self> {
        let dynamic_game_objects =
            create_chunk(SceneKind::DynamicGameObject, info.dynamic_game_objects)?;

        let mut static_chunks = Vec::with_capacity(info.static_chunks);
        for i in 0..info.static_chunks {
            let kind = SceneKind::StaticGameObjct { chunk: i };
            let game_objects = create_chunk(kind, info.static_game_objects_per_chunk)?;
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
        let SceneId { kind, index } = handle.scene_id();

        let Ok(chunk) = self.find_chunk::<T>(kind) else {
            return;
        };
        let ptr = &chunk[index];
        ptr.borrow_mut().is_alive = false;
    }

    pub fn deref_component(
        &self,
        handle: DynComponentHandle,
    ) -> EcsResult<()> {
        let SceneId { kind, index } = handle.scene_id();
        let ecs_type_id = handle.ecs_type_id();

        if kind != SceneKind::Component {
            return Err(EcsError::InvalidCast);
        }

        match ecs_type_id {
            EcsTypeId::MeshRendererComponent => {
                let chunk = &self.mesh_renderer_components;
                let aref = &chunk[index].borrow();

            }
            ecs_type_id => ris_error::throw!(
                "ecs type {:?} is not a component, and thus is not assigned to a game object",
                ecs_type_id
            ),
        }

        panic!()
    }

    pub fn find_game_object_of_component(
        &self,
        handle: DynComponentHandle,
    ) -> EcsResult<GameObjectHandle> {
        let kind = handle.scene_id().kind;
        let ecs_type_id = handle.ecs_type_id();

        if kind != SceneKind::Component {
            return Err(EcsError::InvalidCast);
        }

        let game_object = match ecs_type_id {
            EcsTypeId::MeshRendererComponent => {
                let generic_handle =
                    GenericHandle::<MeshRendererComponent>::from_dyn(handle.into());
                let generic =
                    ris_error::unwrap!(generic_handle, "handle was not a mesh component",);

                let ptr = self.deref(generic)?;
                let aref = ptr.borrow();
                aref.game_object()
            }
            EcsTypeId::ScriptComponent => {
                let generic_handle = GenericHandle::<DynScriptComponent>::from_dyn(handle.into());
                let generic =
                    ris_error::unwrap!(generic_handle, "handle was not a scrip component",);

                let ptr = self.deref(generic)?;
                let aref = ptr.borrow();
                aref.game_object()
            }
            ecs_type_id => ris_error::throw!(
                "ecs type {:?} is not a component, and thus is not assigned to a game object",
                ecs_type_id
            ),
        };

        Ok(game_object)
    }

    pub fn destroy_component(&self, handle: DynComponentHandle) {
        let kind = handle.scene_id().kind;
        let ecs_type_id = handle.ecs_type_id();

        if kind != SceneKind::Component {
            return;
        }

        match ecs_type_id {
            EcsTypeId::MeshRendererComponent => {
                let generic_handle =
                    GenericHandle::<MeshRendererComponent>::from_dyn(handle.into());
                let generic =
                    ris_error::unwrap!(generic_handle, "handle was not a mesh component",);

                self.destroy_component_inner(generic);
            }
            EcsTypeId::ScriptComponent => {
                let generic_handle = GenericHandle::<DynScriptComponent>::from_dyn(handle.into());
                let generic =
                    ris_error::unwrap!(generic_handle, "handle was not a scrip component",);

                self.destroy_component_inner(generic);
            }
            ecs_type_id => ris_error::throw!("ecs type {:?} is not a component", ecs_type_id),
        }
    }

    fn find_chunk<T: EcsObject>(&self, kind: SceneKind) -> EcsResult<&[EcsPtr<T>]> {
        match kind {
            SceneKind::Null => Err(EcsError::IsNull),
            SceneKind::DynamicGameObject => cast(&self.dynamic_game_objects),
            SceneKind::StaticGameObjct { chunk } => cast(&self.static_chunks[chunk].game_objects),
            SceneKind::Component => match T::ecs_type_id() {
                EcsTypeId::MeshRendererComponent => cast(&self.mesh_renderer_components),
                EcsTypeId::ScriptComponent => cast(&self.script_components),
                _ => Err(EcsError::TypeDoesNotMatchSceneKind),
            },
            SceneKind::Other => match T::ecs_type_id() {
                EcsTypeId::VideoMesh => cast(&self.video_meshes),
                _ => Err(EcsError::TypeDoesNotMatchSceneKind),
            },
        }
    }

    fn destroy_component_inner<T: Component>(&self, handle: GenericHandle<T>) {
        let SceneId { kind, index } = handle.scene_id();

        let Ok(chunk) = self.find_chunk::<T>(kind) else {
            return;
        };
        let ptr = &chunk[index];
        let mut aref_mut = ptr.borrow_mut();
        aref_mut.destroy(self);
        aref_mut.is_alive = false;
    }
}

fn create_chunk<T: EcsObject>(kind: SceneKind, capacity: usize) -> EcsResult<Vec<EcsPtr<T>>> {
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

fn cast<T: EcsObject, U: EcsObject>(chunk: &[EcsPtr<T>]) -> EcsResult<&[EcsPtr<U>]> {
    if T::ecs_type_id() != U::ecs_type_id() {
        return Err(EcsError::InvalidCast);
    }

    // transmute is safe, because T is equal to U
    let result = unsafe { std::mem::transmute::<&[EcsPtr<T>], &[EcsPtr<U>]>(chunk) };

    Ok(result)
}

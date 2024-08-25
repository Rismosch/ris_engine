use crate::ptr::ArefCell;
use crate::ptr::StrongPtr;
use crate::ptr::WeakPtr;

use super::game_object::GameObject;
use super::id::EcsObject;
use super::id::GameObjectId;
use super::id::GameObjectKind;
use super::id::Handle;
use super::id::EcsId;
use super::mesh_component::MeshComponent;

const DEFAULT_MOVABLE_GAME_OBJECTS: usize = 1024;
const DEFAULT_STATIC_CHUNKS: usize = 8;
const DEFAULT_STATIC_GAME_OBJECTS_PER_CHUNK: usize = 1024;
const DEFAULT_MESH_COMPONENTS: usize = 1024;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SceneError {
    ObjectIsDestroyed,
    ScaleMustBePositive,
    CircularHierarchy,
    OutOfBounds,
    OutOfMemory,
    TypeDoesNotMatchId,
    InvalidCast,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SceneCreateInfo {
    pub movable_game_objects: usize,
    pub static_chunks: usize,
    pub static_game_objects_per_chunk: usize,
    pub mesh_components: usize,
}

pub struct Scene {
    pub movable_game_objects: Vec<StrongPtr<ArefCell<GameObject>>>,
    pub static_game_objects: Vec<Vec<StrongPtr<ArefCell<GameObject>>>>,
    pub mesh_components: Vec<StrongPtr<ArefCell<MeshComponent>>>,
}

impl std::fmt::Display for SceneError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            SceneError::ObjectIsDestroyed => write!(f, "object is destroyed"),
            SceneError::ScaleMustBePositive => write!(f, "scale must be larger than 0"),
            SceneError::CircularHierarchy => {
                write!(f, "operation would have caused a circular hierarchy")
            }
            SceneError::OutOfBounds => write!(f, "operation was out of bounds"),
            SceneError::OutOfMemory => write!(f, "out of memory"),
            SceneError::TypeDoesNotMatchId => write!(f, "type does not match the id"),
            SceneError::InvalidCast => write!(f, "cast is invalid"),
        }
    }
}

pub type SceneResult<T> = Result<T, SceneError>;

impl std::error::Error for SceneError {}

impl Default for SceneCreateInfo {
    fn default() -> Self {
        Self {
            movable_game_objects: DEFAULT_MOVABLE_GAME_OBJECTS,
            static_chunks: DEFAULT_STATIC_CHUNKS,
            static_game_objects_per_chunk: DEFAULT_STATIC_GAME_OBJECTS_PER_CHUNK,
            mesh_components: DEFAULT_MESH_COMPONENTS,
        }
    }
}

impl Scene {
    pub fn new(info: SceneCreateInfo) -> SceneResult<Self> {
        let mut movable_game_objects = Vec::with_capacity(info.movable_game_objects);
        for i in 0..info.movable_game_objects {
            let id = GameObjectId {
                kind: GameObjectKind::Movable,
                index: i,
            };
            let handle = Handle::from(id.into(), 0)?;
            let game_object = GameObject::new(handle, false);
            let ptr = StrongPtr::new(ArefCell::new(game_object));
            movable_game_objects.push(ptr);
        }

        let mut static_game_objects = Vec::with_capacity(info.static_chunks);
        for i in 0..info.static_chunks {
            let mut chunk = Vec::with_capacity(info.static_game_objects_per_chunk);
            for j in 0..info.static_game_objects_per_chunk {
                let id = GameObjectId {
                    kind: GameObjectKind::Static { chunk: i },
                    index: j,
                };
                let handle = Handle::from(id.into(), 0)?;
                let game_object = GameObject::new(handle, false);
                let ptr = StrongPtr::new(ArefCell::new(game_object));
                chunk.push(ptr);
            }

            static_game_objects.push(chunk);
        }

        let mut mesh_components = Vec::with_capacity(info.mesh_components);
        for i in 0..info.mesh_components {
            let handle = Handle::from(i.into(), 0)?;
            let visual_mesh = MeshComponent::new(handle, false);
            let ptr = StrongPtr::new(ArefCell::new(visual_mesh));
            mesh_components.push(ptr);
        }

        Ok(Self {
            movable_game_objects,
            static_game_objects,
            mesh_components,
        })
    }

    pub fn resolve<T: EcsObject>(&self, handle: Handle<T>) -> SceneResult<WeakPtr<ArefCell<T>>> {
        let ptr: WeakPtr<ArefCell<T>> = match handle.id {
            EcsId::GameObject(GameObjectId { kind, index }) => match kind {
                GameObjectKind::Static { chunk } => cast(&self.static_game_objects[chunk][index])?,
                GameObjectKind::Movable => cast(&self.movable_game_objects[index])?,
            },
            EcsId::Index(index) => match T::ecs_type_id() {
                super::id::ECS_TYPE_ID_MESH_COMPONENT => cast(&self.mesh_components[index])?,
                //id::ECS_TYPE_ID_SCRIPT_COMPONENT => (),
                _ => return Err(SceneError::OutOfBounds),
            },
        };

        let aref = ptr.borrow();

        let is_alive = aref.is_alive();
        let generation_matches = aref.handle().generation == handle.generation;

        if is_alive && generation_matches {
            Ok(ptr)
        } else {
            Err(SceneError::ObjectIsDestroyed)
        }
    }

    pub fn count_available_game_objects(&self, kind: GameObjectKind) -> usize {
        let chunk = match kind {
            GameObjectKind::Movable => &self.movable_game_objects,
            GameObjectKind::Static { chunk } => &self.static_game_objects[chunk],
        };

        chunk.iter().filter(|x| x.borrow().is_alive()).count()
    }
}

fn cast<T: EcsObject, U: EcsObject>(ptr: &StrongPtr<ArefCell<T>>) -> SceneResult<WeakPtr<ArefCell<U>>> {

    // if i haven't fucked up the match logic in Scene::resolve, then T and U should always be the
    // same and the transmute is safe. however, if i do fuck up the logic, then this assert catches
    // it. well, that assumes that i haven't fucked up the ecs type ids, which should properly
    // match their type. oh well. good luck future me.

    #[cfg(debug_assertions)]
    {
        if T::ecs_type_id() != U::ecs_type_id() {
            return Err(SceneError::InvalidCast);
        }
    }

    let weak = ptr.to_weak();
    let result = unsafe {std::mem::transmute::<WeakPtr<ArefCell<T>>, WeakPtr<ArefCell<U>>>(weak)};

    Ok(result)
}


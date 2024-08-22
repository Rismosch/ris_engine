use crate::ptr::ArefCell;
use crate::ptr::StrongPtr;
use crate::ptr::WeakPtr;

use super::game_object::GameObject;
use super::game_object::GameObjectStrongPtr;
use super::game_object::GameObjectWeakPtr;
use super::id::GameObjectHandle;
use super::id::GameObjectId;
use super::id::GameObjectKind;
use super::id::ComponentHandle;
use super::id::ComponentKind;
use super::id::IComponent;
use super::visual_mesh::VisualMesh;

const DEFAULT_MOVABLES: usize = 1024;
const DEFAULT_STATIC_CHUNKS: usize = 8;
const DEFAULT_STATICS_PER_CHUNK: usize = 1024;
const DEFAULT_VISUAL_MESHES: usize = 1024;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SceneError {
    ObjectIsDestroyed,
    ScaleMustBePositive,
    CircularHierarchy,
    IndexOutOfBounds,
    OutOfMemory,
    InvalidCast,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SceneCreateInfo {
    pub movables: usize,
    pub static_chunks: usize,
    pub statics_per_chunk: usize,
    pub visual_meshes: usize,
}

pub struct Scene {
    // game objects
    pub movables: Vec<GameObjectStrongPtr>,
    pub statics: Vec<Vec<GameObjectStrongPtr>>,

    // components
    pub visual_meshes: Vec<StrongPtr<ArefCell<VisualMesh>>>,
}

impl std::fmt::Display for SceneError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            SceneError::ObjectIsDestroyed => write!(f, "object is destroyed"),
            SceneError::ScaleMustBePositive => write!(f, "scale must be larger than 0"),
            SceneError::CircularHierarchy => {
                write!(f, "operation would have caused a circular hierarchy")
            }
            SceneError::IndexOutOfBounds => write!(f, "index was out of bounds"),
            SceneError::OutOfMemory => write!(f, "out of memory"),
            SceneError::InvalidCast => write!(f, "cannot cast component to specified type"),
        }
    }
}

pub type SceneResult<T> = Result<T, SceneError>;

impl std::error::Error for SceneError {}

impl Default for SceneCreateInfo {
    fn default() -> Self {
        Self {
            movables: DEFAULT_MOVABLES,
            static_chunks: DEFAULT_STATIC_CHUNKS,
            statics_per_chunk: DEFAULT_STATICS_PER_CHUNK,
            visual_meshes: DEFAULT_VISUAL_MESHES,
        }
    }
}

impl Scene {
    pub fn new(info: SceneCreateInfo) -> Self {
        let mut movables = Vec::with_capacity(info.movables);
        for i in 0..info.movables {
            let handle = GameObjectHandle {
                id: GameObjectId {
                    kind: GameObjectKind::Movable,
                    index: i,
                },
                generation: 0,
            };

            let game_object = GameObject::new(handle, false);
            let ptr = StrongPtr::new(ArefCell::new(game_object));
            movables.push(ptr);
        }

        let mut statics = Vec::with_capacity(info.static_chunks);
        for i in 0..info.static_chunks {
            let mut chunk = Vec::with_capacity(info.statics_per_chunk);
            for j in 0..info.statics_per_chunk {
                let handle = GameObjectHandle {
                    id: GameObjectId {
                        kind: GameObjectKind::Static { chunk: i },
                        index: j,
                    },
                    generation: 0,
                };

                let game_object = GameObject::new(handle, false);
                let ptr = StrongPtr::new(ArefCell::new(game_object));
                chunk.push(ptr);
            }

            statics.push(chunk);
        }

        let mut visual_meshes = Vec::with_capacity(info.visual_meshes);
        for i in 0..info.visual_meshes {
            let handle = ComponentHandle {
                kind: ComponentKind::VisualMesh,
                index: i,
                generation: 0,
            };

            let visual_mesh = VisualMesh::new(handle, false);
            let ptr = StrongPtr::new(ArefCell::new(visual_mesh));
            visual_meshes.push(ptr);
        }

        Self { movables, statics, visual_meshes }
    }

    pub fn resolve(&self, handle: GameObjectHandle) -> SceneResult<GameObjectWeakPtr> {
        let ptr = match handle.id.kind {
            GameObjectKind::Movable => &self.movables[handle.id.index],
            GameObjectKind::Static { chunk } => &self.statics[chunk][handle.id.index],
        };

        let aref = ptr.borrow();

        let is_alive = aref.is_alive();
        let generation_matches = aref.handle().generation == handle.generation;

        if is_alive && generation_matches {
            Ok(ptr.to_weak())
        } else {
            Err(SceneError::ObjectIsDestroyed)
        }
    }

    pub fn resolve_component<T: IComponent>(&self, handle: ComponentHandle) -> SceneResult<WeakPtr<ArefCell<T>>> {
        let ptr: WeakPtr<ArefCell<T>> = match handle.kind {
            ComponentKind::Script => panic!("not implemented"),
            ComponentKind::VisualMesh => cast(&self.visual_meshes[handle.index])?,
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
            GameObjectKind::Movable => &self.movables,
            GameObjectKind::Static { chunk } => &self.statics[chunk],
        };

        chunk.iter().filter(|x| x.borrow().is_alive()).count()
    }
}

fn cast<T: IComponent, U: IComponent>(strong_ptr_t: &StrongPtr<ArefCell<T>>) -> SceneResult<WeakPtr<ArefCell<U>>> {
    let type_id_t = T::type_id();
    let type_id_u = U::type_id();

    if type_id_t == type_id_u {
        let weak_ptr_t = strong_ptr_t.to_weak();

        // as long as i haven't fucked up the type ids, this should be safe, as T and U are the
        // same type
        let weak_ptr_u = unsafe {
            std::mem::transmute::<WeakPtr<ArefCell<T>>,WeakPtr<ArefCell<U>>>(weak_ptr_t)
        };

        Ok(weak_ptr_u)
    } else {
        Err(SceneError::InvalidCast)
    }
}

use ris_math::matrix::Mat4;
use ris_math::quaternion::Quat;
use ris_math::vector::Vec3;

use crate::cell::ArefCell;
use crate::ptr::StrongPtr;
use crate::ptr::WeakPtr;

use super::game_object::GameObject;
use super::game_object::GameObjectHandle;
use super::game_object::GameObjectStrongPtr;
use super::game_object::GameObjectWeakPtr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SceneId {
    Movable {
        index: usize,
    },
    Static{
        chunk: usize,
        index: usize,
    },
}

impl Default for SceneId {
    fn default() -> Self {
        Self::Movable{
            index: usize::MAX,
        }
    }
}

pub struct Scene {
    movables: Vec<GameObjectStrongPtr>,
    statics: Vec<Vec<GameObjectStrongPtr>>,
}

impl Scene {
    pub fn new(
        movables_len: usize,
        static_chunks: usize,
        statics_per_chunk: usize,
    ) -> Self {
        let mut movables = Vec::with_capacity(movables_len);
        for _ in 0..movables_len {
            let game_object = GameObject::new(0);
            movables.push(game_object);
        }

        let mut statics = Vec::with_capacity(static_chunks);
        for _ in 0..static_chunks {
            let mut chunk = Vec::with_capacity(statics_per_chunk);
            for _ in 0..statics_per_chunk {
                let game_object = GameObject::new(0);
                chunk.push(game_object);
            }

            statics.push(chunk);
        }

        Self {
            movables,
            statics,
        }
    }

    pub fn resolve(&self, handle: GameObjectHandle) -> Option<GameObjectWeakPtr> {
        let ptr = match handle.id {
            SceneId::Movable { index } => &self.movables[index],
            SceneId::Static { chunk, index } => &self.statics[chunk][index],
        };

        let generation = ptr.borrow().generation();

        if generation == handle.generation {
            Some(ptr.to_weak())
        } else {
            None
        }
    }
}

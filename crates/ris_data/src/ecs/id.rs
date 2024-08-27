use super::handle::GenericHandle;
use super::scene::Scene;

//
// ids
//

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameObjectKind {
    Movable,
    Static { chunk: usize },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GameObjectId {
    pub kind: GameObjectKind,
    pub index: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SceneId {
    GameObject(GameObjectId),
    Index(usize),
}

impl From<GameObjectId> for SceneId {
    fn from(value: GameObjectId) -> Self {
        Self::GameObject(value)
    }
}

impl From<usize> for SceneId {
    fn from(value: usize) -> Self {
        Self::Index(value)
    }
}

pub type EcsTypeId = usize;

//
// ecs traits
//

pub trait EcsObject {
    fn ecs_type_id() -> EcsTypeId;
    fn handle(&self) -> GenericHandle<Self>;
    fn is_alive(&self) -> bool;
}

pub trait Component : std::fmt::Debug {
    fn destroy(&mut self, scene: &Scene);
}


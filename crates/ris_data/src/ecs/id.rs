use std::fmt::Debug;

use crate::ptr::ArefCell;
use crate::ptr::StrongPtr;
use crate::ptr::WeakPtr;

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
// ecs traits and objects
//

pub trait EcsObject: Default {
    fn ecs_type_id() -> EcsTypeId;
}

pub trait Component : EcsObject {
    fn destroy(&mut self, scene: &Scene);
}

pub struct EcsInstance<T: EcsObject> {
    pub value: T,
    pub handle: GenericHandle<T>,
    pub is_alive: bool,
}

pub type EcsPtr<T> = StrongPtr<ArefCell<EcsInstance<T>>>;
pub type EcsWeakPtr<T> = WeakPtr<ArefCell<EcsInstance<T>>>;

impl<T: EcsObject> EcsInstance<T> {
    pub fn new(handle: GenericHandle<T>) -> Self {
        Self{
            value: T::default(),
            handle,
            is_alive: false,
        }
    }
}

impl<T: EcsObject> std::ops::Deref for EcsInstance<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T: EcsObject> std::ops::DerefMut for EcsInstance<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}


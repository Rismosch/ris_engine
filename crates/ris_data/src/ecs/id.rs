use std::fmt::Debug;

use imgui::Ui;

use ris_error::RisResult;
use ris_ptr::ArefCell;
use ris_ptr::StrongPtr;
use ris_ptr::WeakPtr;

use crate::gameloop::frame::Frame;
use crate::god_state::GodState;

use super::decl::EcsTypeId;
use super::decl::GameObjectHandle;
use super::error::EcsError;
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
pub enum SceneKind {
    Null,
    MovableGameObject,
    StaticGameObjct { chunk: usize },
    Component,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SceneId {
    pub kind: SceneKind,
    pub index: usize,
}

impl From<GameObjectKind> for SceneKind {
    fn from(value: GameObjectKind) -> Self {
        match value {
            GameObjectKind::Movable => Self::MovableGameObject,
            GameObjectKind::Static { chunk } => Self::StaticGameObjct { chunk },
        }
    }
}

impl TryFrom<SceneKind> for GameObjectKind {
    type Error = EcsError;

    fn try_from(value: SceneKind) -> Result<Self, Self::Error> {
        match value {
            SceneKind::Null => Err(EcsError::IsNull),
            SceneKind::MovableGameObject => Ok(Self::Movable),
            SceneKind::StaticGameObjct { chunk } => Ok(Self::Static { chunk }),
            _ => Err(EcsError::InvalidCast),
        }
    }
}

//
// ecs traits and objects
//

pub trait EcsObject: Debug + Default {
    fn ecs_type_id() -> EcsTypeId;
}

pub trait Component: EcsObject {
    fn create(game_object: GameObjectHandle) -> Self;
    fn destroy(&mut self, scene: &Scene);
    fn game_object(&self) -> GameObjectHandle;
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
        Self {
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

unsafe impl<T: EcsObject> Send for EcsInstance<T> where T: Send {}
unsafe impl<T: EcsObject> Sync for EcsInstance<T> where T: Sync {}

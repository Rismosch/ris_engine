use std::marker::PhantomData;

use super::game_object::GameObject;
use super::mesh_component::MeshComponent;
use super::scene::Scene;
use super::scene::SceneResult;
use super::scene::SceneError;
use super::script_component::ScriptComponent;

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
pub enum EcsId {
    GameObject(GameObjectId),
    Index(usize),
}

impl From<GameObjectId> for EcsId {
    fn from(value: GameObjectId) -> Self {
        Self::GameObject(value)
    }
}

impl From<usize> for EcsId {
    fn from(value: usize) -> Self {
        Self::Index(value)
    }
}

//
// components
//

pub trait EcsObject {
    fn ecs_type_id() -> EcsTypeId;
    fn handle(&self) -> Handle<Self>;
    fn is_alive(&self) -> bool;
}

pub trait Component : std::fmt::Debug {
    fn destroy(&mut self, scene: &Scene);
}

//
// handle
//

#[derive(Debug)]
pub struct Handle<T: EcsObject + ?Sized> {
    pub id: EcsId,
    pub generation: usize,
    boo: PhantomData<T>,
}

impl<T: EcsObject> Handle<T> {
    pub fn from(id: EcsId, generation: usize) -> SceneResult<Self> {
        // assert the id matches with the type
        let type_matches_id = match id {
            EcsId::GameObject(_) => T::ecs_type_id() == ECS_TYPE_ID_GAME_OBJECT,
            EcsId::Index(_) => T::ecs_type_id() != ECS_TYPE_ID_GAME_OBJECT,
        };

        if !type_matches_id {
            return Err(SceneError::TypeDoesNotMatchId);
        }

        Ok(Self {
            id,
            generation,
            boo: PhantomData::default(),
        })
    }
}

impl<T: EcsObject> Clone for Handle<T> {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            generation: self.generation,
            boo: PhantomData::default(),
        }
    }
}

impl<T: EcsObject> PartialEq for Handle<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id &&
            self.generation == other.generation
    }
}

impl<T: EcsObject> Copy for Handle<T> {}
impl<T: EcsObject> Eq for Handle<T> {}

//
// declarations
//

pub type GameObjectHandle = Handle<GameObject>;
pub type MeshComponentHandle = Handle<MeshComponent>;
pub type ScriptComponentHandle = Handle<ScriptComponent>;

pub type EcsTypeId = usize;

pub const ECS_TYPE_ID_GAME_OBJECT: EcsTypeId = 0;
pub const ECS_TYPE_ID_MESH_COMPONENT: EcsTypeId = 1;
pub const ECS_TYPE_ID_SCRIPT_COMPONENT: EcsTypeId = 2;


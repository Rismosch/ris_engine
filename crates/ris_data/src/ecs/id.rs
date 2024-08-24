use std::marker::PhantomData;

use super::game_object::GameObject;
use super::mesh_component::MeshComponent;
use super::scene::Scene;
use super::scene::SceneResult;
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
pub struct IndexId {
    pub index: usize,
}

impl IndexId {
    pub fn new(index: usize) -> Self {
        Self {
            index,
        }
    }
}

//
// components
//

pub trait EcsObject<Id> {
    fn handle(&self) -> Handle<Self, Id>;
    fn is_alive(&self) -> bool;
}

pub trait Component : std::fmt::Debug {
    fn destroy(&mut self, scene: &Scene);
}

//
// handle
//

#[derive(Debug)]
pub struct Handle<T: ?Sized, Id> {
    pub id: Id,
    pub generation: usize,
    boo: PhantomData<T>,
}

impl<T, Id> Handle<T, Id> {
    pub fn from(id: Id, generation: usize) -> Self {
        Self {
            id,
            generation,
            boo: PhantomData::default(),
        }
    }
}

impl<T, Id: Clone> Clone for Handle<T, Id> {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            generation: self.generation,
            boo: PhantomData::default(),
        }
    }
}

impl<T, Id: PartialEq> PartialEq for Handle<T, Id> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id &&
            self.generation == other.generation
    }
}

impl<T, Id: Copy> Copy for Handle<T, Id> {}
impl<T, Id: Eq> Eq for Handle<T, Id> {}


//
// handle declarations
//

pub type GameObjectHandle = Handle<GameObject, GameObjectId>;
pub type MeshComponentHandle = Handle<MeshComponent, IndexId>;
pub type ScriptComponentHandle = Handle<ScriptComponent, IndexId>;

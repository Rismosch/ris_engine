use std::fmt::Debug;
use std::marker::PhantomData;

use super::id::Component;
use super::id::EcsObject;
use super::id::EcsTypeId;
use super::id::SceneId;
use super::id::SceneKind;
use super::error::EcsError;
use super::error::EcsResult;
use super::scene::Scene;

//
// handles
//

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DynHandle {
    ecs_type_id: EcsTypeId,
    scene_id: SceneId,
    generation: usize,
}

#[derive(Debug)]
pub struct GenericHandle<T: EcsObject + ?Sized> {
    inner: DynHandle,
    boo: PhantomData<T>,
}

pub trait Handle : Debug {
    fn ecs_type_id() -> EcsTypeId where Self: Sized;
    fn to_dyn(self) -> DynHandle;
}

pub trait ComponentHandle : Handle {}

// 
// constructors
//

impl DynHandle {
    pub fn new(ecs_type_id: EcsTypeId, scene_id: SceneId, generation: usize) -> EcsResult<Self> {
        // assert the ecs_type_id matches with the scene_id
        let type_matches_id = match scene_id.kind {
            SceneKind::Null => true,
            SceneKind::StaticGameObjct { .. } => ecs_type_id == super::decl::ECS_TYPE_ID_GAME_OBJECT,
            SceneKind::MovableGameObject => ecs_type_id == super::decl::ECS_TYPE_ID_GAME_OBJECT,
            SceneKind::Component => ecs_type_id != super::decl::ECS_TYPE_ID_GAME_OBJECT,
        };

        if type_matches_id {
            Ok(Self{
                ecs_type_id,
                scene_id,
                generation,
            })
        } else {
            return Err(EcsError::TypeDoesNotMatchId);
        }
    }

    pub fn null(ecs_type_id: EcsTypeId) -> Self {
        Self{
            ecs_type_id,
            scene_id: SceneId {
                kind: SceneKind::Null,
                index: 0,
            },
            generation: 0,
        }
    }
}

impl<T: EcsObject> From<GenericHandle<T>> for DynHandle {
    fn from(value: GenericHandle<T>) -> Self {
        value.inner
    }
}

impl<T: EcsObject + ?Sized> GenericHandle<T> {
    pub fn new(scene_id: SceneId, generation: usize) -> EcsResult<Self> {
        let inner = DynHandle::new(T::ecs_type_id(), scene_id, generation)?;
        Ok(Self {
            inner,
            boo: PhantomData::default(),
        })
    }

    pub fn null() -> Self {
        let inner = DynHandle::null(T::ecs_type_id());
        Self {
            inner,
            boo: PhantomData::default(),
        }
    }
}

impl<T: EcsObject> GenericHandle<T> {
    pub fn from_dyn(value: DynHandle) -> EcsResult<Self> {
        if T::ecs_type_id() == value.ecs_type_id {
            Ok(GenericHandle {
                inner: value,
                boo: PhantomData::default(),
            })
        } else {
            Err(EcsError::TypeDoesNotMatchId)
        }
    }

    pub fn from_handle(value: impl Handle) -> EcsResult<Self> {
        let dyn_handle = value.to_dyn();
        Self::from_dyn(dyn_handle)
    }
}

//
// components
//

impl DynHandle {
    pub fn ecs_type_id(self) -> EcsTypeId {
        self.ecs_type_id
    }

    pub fn scene_id(self) -> SceneId {
        self.scene_id
    }

    pub fn generation(self) -> usize {
        self.generation
    }
}

impl<T: EcsObject + ?Sized> GenericHandle<T> {
    pub fn ecs_type_id(self) -> EcsTypeId {
        self.inner.ecs_type_id()
    }
}

//
// trait implementations
//

impl<T: EcsObject> std::ops::Deref for GenericHandle<T> {
    type Target = DynHandle;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T: EcsObject> std::ops::DerefMut for GenericHandle<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<T: EcsObject> Clone for GenericHandle<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            boo: PhantomData::default(),
        }
    }
}

impl<T: EcsObject> PartialEq for GenericHandle<T> {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl<T: EcsObject> Copy for GenericHandle<T> {}
impl<T: EcsObject> Eq for GenericHandle<T> {}

impl<T: EcsObject> Handle for GenericHandle<T> {
    fn ecs_type_id() -> EcsTypeId {
        T::ecs_type_id()
    }

    fn to_dyn(self) -> DynHandle {
        self.inner
    }
}

impl<T: Component> ComponentHandle for GenericHandle<T> {}

//
// common functions
//

impl<T: EcsObject> GenericHandle<T> {
    pub fn is_alive(self, scene: &Scene) -> bool {
        scene.deref(self).is_ok()
    }
}

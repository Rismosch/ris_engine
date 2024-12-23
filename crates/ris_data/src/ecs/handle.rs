use std::fmt::Debug;
use std::marker::PhantomData;

use super::decl::EcsTypeId;
use super::decl::GameObjectHandle;
use super::error::EcsError;
use super::error::EcsResult;
use super::id::Component;
use super::id::EcsObject;
use super::id::SceneId;
use super::id::SceneKind;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DynComponentHandle {
    inner: DynHandle,
}

#[derive(Debug)]
pub struct GenericHandle<T: EcsObject + ?Sized> {
    inner: DynHandle,
    boo: PhantomData<T>,
}

pub trait Handle: Debug + Clone + Copy {
    fn ecs_type_id() -> EcsTypeId
    where
        Self: Sized;
    fn to_dyn(self) -> DynHandle;
}

pub trait ComponentHandle: Handle {
    fn to_dyn_component(self) -> DynComponentHandle;

    fn game_object(self, scene: &Scene) -> EcsResult<GameObjectHandle> {
        let dyn_component = self.to_dyn_component();
        scene.find_game_object_of_component(dyn_component)
    }

    fn destroy(self, scene: &Scene) {
        let Ok(game_object) = self.game_object(scene) else {
            return;
        };

        let dyn_component = self.to_dyn_component();

        game_object.remove_and_destroy_component(scene, dyn_component);
    }
}

//
// constructors
//

impl DynHandle {
    pub fn new(ecs_type_id: EcsTypeId, scene_id: SceneId, generation: usize) -> EcsResult<Self> {
        if ecs_type_id.matches(scene_id.kind) {
            Ok(Self {
                ecs_type_id,
                scene_id,
                generation,
            })
        } else {
            Err(EcsError::TypeDoesNotMatchSceneKind)
        }
    }

    pub fn null(ecs_type_id: EcsTypeId) -> Self {
        Self {
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

impl From<DynComponentHandle> for DynHandle {
    fn from(value: DynComponentHandle) -> Self {
        value.inner
    }
}

impl<T: Component> From<GenericHandle<T>> for DynComponentHandle {
    fn from(value: GenericHandle<T>) -> Self {
        Self {
            inner: value.to_dyn(),
        }
    }
}

impl<T: EcsObject + ?Sized> GenericHandle<T> {
    pub fn new(scene_id: SceneId, generation: usize) -> EcsResult<Self> {
        let inner = DynHandle::new(T::ecs_type_id(), scene_id, generation)?;
        Ok(Self {
            inner,
            boo: PhantomData,
        })
    }

    pub fn null() -> Self {
        let inner = DynHandle::null(T::ecs_type_id());
        Self {
            inner,
            boo: PhantomData,
        }
    }
}

impl<T: EcsObject> GenericHandle<T> {
    pub fn from_dyn(value: DynHandle) -> EcsResult<Self> {
        if T::ecs_type_id() == value.ecs_type_id {
            Ok(GenericHandle {
                inner: value,
                boo: PhantomData,
            })
        } else {
            Err(EcsError::InvalidCast)
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

impl std::ops::Deref for DynComponentHandle {
    type Target = DynHandle;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T: EcsObject> std::ops::Deref for GenericHandle<T> {
    type Target = DynHandle;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T: EcsObject> Clone for GenericHandle<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: EcsObject> Copy for GenericHandle<T> {}

impl<T: EcsObject> PartialEq for GenericHandle<T> {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl<T: EcsObject> Eq for GenericHandle<T> {}

impl<T: EcsObject> Handle for GenericHandle<T> {
    fn ecs_type_id() -> EcsTypeId {
        T::ecs_type_id()
    }

    fn to_dyn(self) -> DynHandle {
        self.inner
    }
}

impl<T: Component> ComponentHandle for GenericHandle<T> {
    fn to_dyn_component(self) -> DynComponentHandle {
        self.into()
    }
}

//
// common functions
//

impl<T: EcsObject> GenericHandle<T> {
    pub fn is_alive(self, scene: &Scene) -> bool {
        scene.deref(self).is_ok()
    }
}

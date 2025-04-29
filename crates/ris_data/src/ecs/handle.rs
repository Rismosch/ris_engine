use std::any::TypeId;
use std::fmt::Debug;
use std::marker::PhantomData;

use super::components::mesh_renderer::MeshRendererComponent;
use super::components::script::DynScriptComponent;
use super::decl::GameObjectHandle;
use super::error::EcsError;
use super::error::EcsResult;
use super::game_object::GameObject;
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
    type_id: TypeId,
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
    fn type_id() -> TypeId
    where
        Self: Sized;
    fn to_dyn(self) -> DynHandle;
}

pub trait ComponentHandle: Handle {
    fn to_dyn_component(self) -> DynComponentHandle;

    fn game_object(self, scene: &Scene) -> EcsResult<GameObjectHandle> {
        scene.deref_component(self.to_dyn_component(), |component| component.game_object())
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
    pub fn new(type_id: TypeId, scene_id: SceneId, generation: usize) -> EcsResult<Self> {
        let scene_kind = scene_id.kind;
        let matches = match scene_kind {
            SceneKind::Null => true,
            SceneKind::DynamicGameObject if type_id == TypeId::of::<GameObject>() => true,
            SceneKind::StaticGameObjct { chunk: _ } if type_id == TypeId::of::<GameObject>() => {
                true
            }
            SceneKind::Component if type_id == TypeId::of::<MeshRendererComponent>() => true,
            SceneKind::Component if type_id == TypeId::of::<DynScriptComponent>() => true,
            _ => false,
        };

        if matches {
            Ok(Self {
                type_id,
                scene_id,
                generation,
            })
        } else {
            Err(EcsError::TypeDoesNotMatchSceneKind)
        }
    }

    pub fn null(type_id: TypeId) -> Self {
        Self {
            type_id,
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

impl<T: Component + 'static> From<GenericHandle<T>> for DynComponentHandle {
    fn from(value: GenericHandle<T>) -> Self {
        Self {
            inner: value.to_dyn(),
        }
    }
}

impl<T: EcsObject + ?Sized + 'static> GenericHandle<T> {
    pub fn new(scene_id: SceneId, generation: usize) -> EcsResult<Self> {
        let inner = DynHandle::new(TypeId::of::<T>(), scene_id, generation)?;
        Ok(Self {
            inner,
            boo: PhantomData,
        })
    }

    pub fn null() -> Self {
        let inner = DynHandle::null(TypeId::of::<T>());
        Self {
            inner,
            boo: PhantomData,
        }
    }
}

impl<T: EcsObject + 'static> GenericHandle<T> {
    pub fn from_dyn(value: DynHandle) -> EcsResult<Self> {
        if TypeId::of::<T>() == value.type_id {
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
    pub fn type_id(self) -> TypeId {
        self.type_id
    }

    pub fn scene_id(self) -> SceneId {
        self.scene_id
    }

    pub fn generation(self) -> usize {
        self.generation
    }
}

impl<T: EcsObject + ?Sized> GenericHandle<T> {
    pub fn type_id(self) -> TypeId {
        self.inner.type_id()
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

impl<T: EcsObject + 'static> Handle for GenericHandle<T> {
    fn type_id() -> TypeId {
        TypeId::of::<T>()
    }

    fn to_dyn(self) -> DynHandle {
        self.inner
    }
}

impl<T: Component + 'static> ComponentHandle for GenericHandle<T> {
    fn to_dyn_component(self) -> DynComponentHandle {
        self.into()
    }
}

//
// common functions
//

impl<T: EcsObject + 'static> GenericHandle<T> {
    pub fn is_alive(self, scene: &Scene) -> bool {
        scene.deref(self).is_ok()
    }
}

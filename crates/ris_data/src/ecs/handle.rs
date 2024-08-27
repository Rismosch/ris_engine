use std::marker::PhantomData;

use super::game_object::GameObject;
use super::id::EcsId;
use super::id::EcsObject;
use super::id::EcsTypeId;
use super::mesh_component::MeshComponent;
use super::error::EcsError;
use super::error::EcsResult;
use super::script_component::ScriptComponent;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DynHandle {
    pub id: EcsId,
    pub generation: usize,
}

#[derive(Debug)]
pub struct GenericHandle<T: EcsObject + ?Sized> {
    inner: DynHandle,
    boo: PhantomData<T>,
}

impl<T: EcsObject> std::ops::Deref for GenericHandle<T> {
    type Target = DynHandle;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

pub trait Handle {
    fn ecs_type_id() -> EcsTypeId where Self: Sized;
    fn to_dyn(self) -> DynHandle;
}

pub trait ComponentHandle : Handle {}

impl<T: EcsObject> std::ops::DerefMut for GenericHandle<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<T: EcsObject> GenericHandle<T> {
    pub fn new(id: EcsId, generation: usize) -> EcsResult<Self> {
        // assert the id matches with the type
        let type_matches_id = match id {
            EcsId::GameObject(_) => T::ecs_type_id() == ECS_TYPE_ID_GAME_OBJECT,
            EcsId::Index(_) => T::ecs_type_id() != ECS_TYPE_ID_GAME_OBJECT,
        };

        if !type_matches_id {
            return Err(EcsError::TypeDoesNotMatchId);
        }

        Ok(Self {
            inner: DynHandle {
                id,
                generation,
            },
            boo: PhantomData::default(),
        })
    }

    pub fn ecs_type_id(self) -> EcsTypeId {
        T::ecs_type_id()
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

//
// declarations
//

macro_rules! decl_handle {
    (
        $handle_name:ident,
        $handle_type:ident,
        $ecs_type_id:expr $(,)?
    ) => {

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct $handle_name(pub GenericHandle<$handle_type>);

        impl std::ops::Deref for $handle_name {
            type Target = GenericHandle<$handle_type>;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl std::ops::DerefMut for $handle_name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }

        impl Handle for $handle_name {
            fn ecs_type_id() -> EcsTypeId {
                $ecs_type_id
            }

            fn to_dyn(self) -> DynHandle {
                self.0.inner
            }
        }

        impl From<GenericHandle<$handle_type>> for $handle_name {
            fn from(value: GenericHandle<$handle_type>) -> Self {
                Self(value)
            }
        }

        impl From<$handle_name> for GenericHandle<$handle_type> {
            fn from(value: $handle_name) -> Self {
                value.0
            }
        }
    }
}

macro_rules! decl_component_handle {
    (
        $handle_name:ident,
        $handle_type:ident,
        $ecs_type_id:expr $(,)?
    ) => {
        decl_handle!($handle_name, $handle_type, $ecs_type_id);

        impl ComponentHandle for $handle_name {}
    }
}

pub const ECS_TYPE_ID_GAME_OBJECT: EcsTypeId = 0;
pub const ECS_TYPE_ID_MESH_COMPONENT: EcsTypeId = 1;
pub const ECS_TYPE_ID_SCRIPT_COMPONENT: EcsTypeId = 2;

decl_handle!(
    GameObjectHandle,
    GameObject,
    ECS_TYPE_ID_GAME_OBJECT,
);
decl_component_handle!(
    MeshComponentHandle,
    MeshComponent,
    ECS_TYPE_ID_MESH_COMPONENT,
);
decl_component_handle!(
    ScriptComponentHandle,
    ScriptComponent,
    ECS_TYPE_ID_SCRIPT_COMPONENT,
);


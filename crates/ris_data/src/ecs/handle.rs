use std::marker::PhantomData;

use super::id::EcsObject;
use super::id::EcsTypeId;
use super::id::SceneId;
use super::error::EcsError;
use super::error::EcsResult;

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

pub trait Handle : std::fmt::Debug {
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
        let type_matches_id = match scene_id {
            SceneId::GameObject(_) => ecs_type_id == super::decl::ECS_TYPE_ID_GAME_OBJECT,
            SceneId::Index(_) => ecs_type_id != super::decl::ECS_TYPE_ID_GAME_OBJECT,
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
}

impl<T: EcsObject> TryFrom<DynHandle> for GenericHandle<T> {
    type Error = EcsError;

    fn try_from(value: DynHandle) -> Result<Self, Self::Error> {
        if T::ecs_type_id() == value.ecs_type_id {
            Ok(GenericHandle {
                inner: value,
                boo: PhantomData::default(),
            })
        } else {
            Err(EcsError::TypeDoesNotMatchId)
        }
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

    //pub fn cast<T: EcsObject>(self) -> EcsResult<GenericHandle<T>> {
    //    if T::ecs_type_id() == self.ecs_type_id {
    //        Ok(GenericHandle {
    //            inner: self,
    //            boo: PhantomData::default(),
    //        })
    //    } else {
    //        Err(EcsError::TypeDoesNotMatchId)
    //    }
    //}
}

impl<T: EcsObject + ?Sized> GenericHandle<T> {
    pub fn ecs_type_id(self) -> EcsTypeId {
        self.ecs_type_id()
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


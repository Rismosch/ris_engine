use std::fmt::Debug;
use std::marker::PhantomData;

use ris_debug::sid::Sid;
use ris_error::Extensions;
use ris_error::RisResult;

use crate::ecs::error::EcsError;
use crate::ecs::error::EcsResult;
use crate::ecs::decl::GameObjectHandle;
use crate::ecs::decl::DynScriptComponentHandle;
use crate::ecs::id::Component;
use crate::ecs::id::EcsInstance;
use crate::ecs::scene::Scene;
use crate::gameloop::frame::Frame;
use crate::god_state::GodState;
use crate::ptr::Aref;
use crate::ptr::ArefMut;

pub mod prelude {
    pub use ris_debug::sid::Sid;

    pub use super::Script;
    pub use super::ScriptEndData;
    pub use super::ScriptStartData;
    pub use super::ScriptUpdateData;
}

pub struct ScriptStartData<'a> {
    pub game_object: GameObjectHandle,
    pub scene: &'a Scene,
}

pub struct ScriptUpdateData<'a> {
    pub game_object: GameObjectHandle,
    pub frame: Frame,
    pub state: &'a GodState,
}

pub struct ScriptEndData<'a> {
    pub game_object: GameObjectHandle,
    pub scene: &'a Scene,
}

pub trait Script: Debug + Send + Sync {
    fn id() -> Sid where Self: Sized;
    fn start(data: ScriptStartData) -> RisResult<Self> where Self: Sized;
    fn update(&mut self, data: ScriptUpdateData) -> RisResult<()>;
    fn end(&mut self, data: ScriptEndData) -> RisResult<()>;
}

#[derive(Debug)]
pub struct DynScript {
    boxxed: Box<dyn Script>,
    id: Sid,
}

#[derive(Debug)]
pub struct DynScriptComponent {
    game_object: GameObjectHandle,
    script: Option<DynScript>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScriptComponentHandle<T: Script> {
    handle: DynScriptComponentHandle,
    boo: PhantomData<T>,
}

pub struct ScriptComponentRef<T: Script> {
    reference: Aref<EcsInstance<DynScriptComponent>>,
    boo: PhantomData<T>,
}

pub struct ScriptComponentRefMut<T: Script> {
    reference: ArefMut<EcsInstance<DynScriptComponent>>,
    boo: PhantomData<T>,
}

impl Default for DynScriptComponent {
    fn default() -> Self {
        Self {
            game_object: GameObjectHandle::null(),
            script: None,
        }
    }
}

impl Component for DynScriptComponent {
    fn create(game_object: GameObjectHandle) -> Self {
        Self {
            game_object,
            ..Default::default()
        }
    }

    fn destroy(&mut self, scene: &Scene) {
        let Some(mut script) = self.script.take() else {
            return;
        };

        let data = ScriptEndData {
            game_object: self.game_object,
            scene,
        };

        if let Err(e) = script.boxxed.end(data) {
            ris_log::error!("failed to end script {:?}: {}", script, e);
        }
    }

    fn game_object(&self) -> GameObjectHandle {
        self.game_object
    }
}

impl DynScriptComponent {
    pub fn update(&mut self, frame: Frame, state: &GodState) -> RisResult<()> {
        let data = ScriptUpdateData {
            game_object: self.game_object,
            frame,
            state,
        };

        match self.script.as_mut() {
            Some(script) => script.boxxed.update(data),
            None => ris_error::new_result!(
                "attempted to call update on a script that hasn't been started yet"
            ),
        }
    }

    pub fn end(&mut self, scene: &Scene) -> RisResult<()> {
        let data = ScriptEndData {
            game_object: self.game_object,
            scene,
        };

        match self.script.as_mut() {
            Some(script) => script.boxxed.end(data),
            None => ris_error::new_result!(
                "attempted to call end on a script that hasn't been started yet"
            ),
        }
    }
}

impl<T: Script + 'static> ScriptComponentHandle<T> {
    pub fn new(scene: &Scene, game_object: GameObjectHandle) -> RisResult<Self> {
        let handle: DynScriptComponentHandle = game_object.add_component(&scene)?.into();

        let data = ScriptStartData { game_object, scene};
        let script = T::start(data)?;

        let ptr = scene.deref(handle.into())?;
        ptr.borrow_mut().script = Some(DynScript{
            boxxed: Box::new(script),
            id: T::id(),
        });

        let generic_handle = Self {
            handle,
            boo: PhantomData::default(),
        };

        Ok(generic_handle)
    }

    pub fn try_from(handle: DynScriptComponentHandle, scene: &Scene) -> EcsResult<Self> {
        let ptr = scene.deref(handle.into())?;
        let aref = ptr.borrow();
        let Some(script) = &aref.script else {
            return Err(EcsError::InvalidOperation("script component was not started".to_string()));
        };

        if T::id() != script.id {
            return Err(EcsError::InvalidCast);
        }

        let generic_handle = Self {
            handle,
            boo: PhantomData::default(),
        };

        Ok(generic_handle)
    }

    pub fn dyn_handle(self) -> DynScriptComponentHandle {
        self.handle
    }

    pub fn script(
        self,
        scene: &Scene,
    ) -> RisResult<ScriptComponentRef<T>> {
        let ptr = scene.deref(self.handle.into())?;
        let aref = ptr.borrow();

        Ok(ScriptComponentRef {
            reference: aref,
            boo: PhantomData::default(),
        })
    }

    pub fn script_mut(
        self,
        scene: &Scene,
    ) -> RisResult<ScriptComponentRefMut<T>> {
        let ptr = scene.deref(self.handle.into())?;
        let aref_mut = ptr.borrow_mut();

        Ok(ScriptComponentRefMut{
            reference: aref_mut,
            boo: PhantomData::default(),
        })
    }
}

impl<T: Script + Default + 'static> std::ops::Deref for ScriptComponentRef<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        let script = ris_error::unwrap!(
            self.reference.script.as_ref().unroll(),
            "script component did not store a script",
        );
        let deref = script.boxxed.deref();

        let dyn_ptr = deref as *const dyn Script;
        let t_ptr = dyn_ptr as *const T;

        // this is safe, because the constructor ensures that the script is of type T
        let reference = unsafe { t_ptr.as_ref() };

        ris_error::unwrap!(
            reference.unroll(),
            "honestly, something is very wrong if reference manages to be none",
        )
    }
}

impl<T: Script + Default + 'static> std::ops::Deref for ScriptComponentRefMut<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        let script = ris_error::unwrap!(
            self.reference.script.as_ref().unroll(),
            "script component did not store a script",
        );
        let deref = script.boxxed.deref();

        let dyn_ptr = deref as *const dyn Script;
        let t_ptr = dyn_ptr as *const T;

        // this is safe, because the constructor ensures that the script is of type T
        let reference = unsafe { t_ptr.as_ref() };

        ris_error::unwrap!(
            reference.unroll(),
            "honestly, something is very wrong if reference manages to be none",
        )
    }
}

impl<T: Script + Default + 'static> std::ops::DerefMut for ScriptComponentRefMut<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let script = ris_error::unwrap!(
            self.reference.script.as_mut().unroll(),
            "script component did not store a script",
        );
        let deref = script.boxxed.deref_mut();

        let dyn_ptr = deref as *mut dyn Script;
        let t_ptr = dyn_ptr as *mut T;

        // this is safe, because the constructor ensures that the script is of type T
        let reference = unsafe {t_ptr.as_mut()};

        ris_error::unwrap!(
            reference.unroll(),
            "honestly, something is very wrong if reference manages to be none",
        )
    }
}

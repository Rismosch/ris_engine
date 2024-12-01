use std::fmt::Debug;
use std::marker::PhantomData;

use imgui::Ui;

use ris_debug::sid::Sid;
use ris_error::Extensions;
use ris_error::RisResult;
use ris_io::serializable::ISerializable;
use ris_ptr::Aref;
use ris_ptr::ArefMut;

use crate::ecs::decl::DynScriptComponentHandle;
use crate::ecs::decl::GameObjectHandle;
use crate::ecs::decl::ScriptComponentHandle;
use crate::ecs::error::EcsError;
use crate::ecs::error::EcsResult;
use crate::ecs::handle::ComponentHandle;
use crate::ecs::id::Component;
use crate::ecs::id::EcsInstance;
use crate::ecs::scene::Scene;
use crate::gameloop::frame::Frame;
use crate::god_state::GodState;

pub struct ScriptStartEndData<'a> {
    pub game_object: GameObjectHandle,
    pub scene: &'a Scene,
}

pub struct ScriptUpdateData<'a> {
    pub game_object: GameObjectHandle,
    pub frame: Frame,
    pub state: &'a GodState,
}

pub struct ScriptInspectData<'a> {
    pub ui: &'a Ui,
    pub game_object: GameObjectHandle,
    pub frame: Frame,
    pub state: &'a GodState,
}

pub trait Script: Debug + Send + Sync + ISerializable {
    fn id() -> Sid where Self: Sized;
    fn name(&self) -> &'static str;
    fn start(&mut self, data: ScriptStartEndData) -> RisResult<()>;
    fn update(&mut self, data: ScriptUpdateData) -> RisResult<()>;
    fn end(&mut self, data: ScriptStartEndData) -> RisResult<()>;
    fn inspect(&mut self, data: ScriptInspectData) -> RisResult<()>;
}

#[derive(Debug)]
pub struct DynScript {
    boxed: Box<dyn Script>,
    id: Sid,
}

#[derive(Debug)]
pub struct DynScriptComponent {
    game_object: GameObjectHandle,
    script: Option<DynScript>,
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

        let data = ScriptStartEndData {
            game_object: self.game_object,
            scene,
        };

        if let Err(e) = script.boxed.end(data) {
            ris_log::error!("failed to end script {:?}: {}", script, e);
        }
    }

    fn game_object(&self) -> GameObjectHandle {
        self.game_object
    }
}

impl DynScriptComponent {
    pub fn game_object(&self) -> GameObjectHandle {
        self.game_object
    }

    pub fn script_mut(&mut self) -> Option<&mut Box<dyn Script>> {
        self.script.as_mut().map(|x| &mut x.boxed)
    }

    pub fn update(&mut self, frame: Frame, state: &GodState) -> RisResult<()> {
        let data = ScriptUpdateData {
            game_object: self.game_object,
            frame,
            state,
        };

        match self.script_mut() {
            Some(script) => script.update(data),
            None => ris_error::new_result!(
                "attempted to call update on a script that hasn't been started yet"
            ),
        }
    }

    pub fn end(&mut self, scene: &Scene) -> RisResult<()> {
        let data = ScriptStartEndData {
            game_object: self.game_object,
            scene,
        };

        match self.script_mut() {
            Some(script) => script.end(data),
            None => ris_error::new_result!(
                "attempted to call end on a script that hasn't been started yet"
            ),
        }
    }
}

impl<T: Script + Default + 'static> ScriptComponentHandle<T> {
    pub fn new(scene: &Scene, game_object: GameObjectHandle) -> RisResult<Self> {
        let handle: DynScriptComponentHandle = game_object.add_component(scene)?.into();

        let data = ScriptStartEndData { game_object, scene };
        let mut script = T::default();
        script.start(data)?;

        let ptr = scene.deref(handle.into())?;
        ptr.borrow_mut().script = Some(DynScript {
            boxed: Box::new(script),
            id: T::id(),
        });

        let generic_handle = Self {
            handle,
            boo: PhantomData,
        };

        Ok(generic_handle)
    }
}

impl<T: Script + 'static> ScriptComponentHandle<T> {
    pub fn try_from(handle: DynScriptComponentHandle, scene: &Scene) -> EcsResult<Self> {
        let ptr = scene.deref(handle.into())?;
        let aref = ptr.borrow();
        let Some(script) = &aref.script else {
            return Err(EcsError::InvalidOperation(
                "script component was not started".to_string(),
            ));
        };

        if T::id() != script.id {
            return Err(EcsError::InvalidCast);
        }

        let generic_handle = Self {
            handle,
            boo: PhantomData,
        };

        Ok(generic_handle)
    }

    pub fn dyn_handle(self) -> DynScriptComponentHandle {
        self.handle
    }

    pub fn game_object(self, scene: &Scene) -> EcsResult<GameObjectHandle> {
        let dyn_handle = self.handle;
        dyn_handle.game_object(scene)
    }

    pub fn destroy(self, scene: &Scene) {
        let dyn_handle = self.handle;
        dyn_handle.destroy(scene)
    }

    pub fn script(self, scene: &Scene) -> RisResult<ScriptComponentRef<T>> {
        let ptr = scene.deref(self.handle.into())?;
        let aref = ptr.borrow();

        Ok(ScriptComponentRef {
            reference: aref,
            boo: PhantomData,
        })
    }

    pub fn script_mut(self, scene: &Scene) -> RisResult<ScriptComponentRefMut<T>> {
        let ptr = scene.deref(self.handle.into())?;
        let aref_mut = ptr.borrow_mut();

        Ok(ScriptComponentRefMut {
            reference: aref_mut,
            boo: PhantomData,
        })
    }
}

impl<T: Script + 'static> std::ops::Deref for ScriptComponentRef<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        let script = ris_error::unwrap!(
            self.reference.script.as_ref().unroll(),
            "script component did not store a script",
        );
        let deref = script.boxed.deref();

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

impl<T: Script + 'static> std::ops::Deref for ScriptComponentRefMut<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        let script = ris_error::unwrap!(
            self.reference.script.as_ref().unroll(),
            "script component did not store a script",
        );
        let deref = script.boxed.deref();

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

impl<T: Script + 'static> std::ops::DerefMut for ScriptComponentRefMut<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let script = ris_error::unwrap!(
            self.reference.script.as_mut().unroll(),
            "script component did not store a script",
        );
        let deref = script.boxed.deref_mut();

        let dyn_ptr = deref as *mut dyn Script;
        let t_ptr = dyn_ptr as *mut T;

        // this is safe, because the constructor ensures that the script is of type T
        let reference = unsafe { t_ptr.as_mut() };

        ris_error::unwrap!(
            reference.unroll(),
            "honestly, something is very wrong if reference manages to be none",
        )
    }
}

use std::fmt::Debug;

use ris_error::Extensions;
use ris_error::RisResult;

use crate::gameloop::frame::Frame;
use crate::god_state::GodState;

use crate::ecs::decl::GameObjectHandle;
use crate::ecs::decl::ScriptComponentHandle;
use crate::ecs::id::Component;
use crate::ecs::scene::Scene;

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
    fn start(&mut self, data: ScriptStartData) -> RisResult<()>;
    fn update(&mut self, data: ScriptUpdateData) -> RisResult<()>;
    fn end(&mut self, data: ScriptEndData) -> RisResult<()>;
}

#[derive(Debug)]
pub struct ScriptComponent {
    game_object: GameObjectHandle,
    script: Option<Box<dyn Script>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GenericScriptComponentHandle<T: Script> {
    handle: ScriptComponentHandle,
    boo: std::marker::PhantomData<T>,
}

impl Default for ScriptComponent {
    fn default() -> Self {
        Self {
            game_object: GameObjectHandle::null(),
            script: None,
        }
    }
}

impl Component for ScriptComponent {
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

        if let Err(e) = script.end(data) {
            ris_log::error!("failed to end script {:?}: {}", script, e);
        }
    }

    fn game_object(&self) -> GameObjectHandle {
        self.game_object
    }
}

impl ScriptComponent {
    pub fn update(&mut self, frame: Frame, state: &GodState) -> RisResult<()> {
        let data = ScriptUpdateData {
            game_object: self.game_object,
            frame,
            state,
        };

        match self.script.as_mut() {
            Some(script) => script.update(data),
            None => ris_error::new_result!(
                "attempted to call update on a script that hasn't been started yet"
            ),
        }
    }
}

impl ScriptComponentHandle {
    pub fn start<T: Script + 'static>(self, scene: &Scene, mut script: T) -> RisResult<()> {
        let ptr = scene.deref(self.into())?;
        let game_object = ptr.borrow().game_object();

        let data = ScriptStartData { game_object, scene };

        script.start(data)?;
        ptr.borrow_mut().script = Some(Box::new(script));

        Ok(())
    }
}

impl<T: Script + Default + 'static> GenericScriptComponentHandle<T> {
    pub fn new(scene: &Scene, game_object: GameObjectHandle) -> RisResult<Self> {
        let handle: ScriptComponentHandle = game_object.add_component(&scene)?.into();
        let script = T::default();
        handle.start(&scene, script)?;

        let generic_handle = Self {
            handle,
            boo: std::marker::PhantomData::default(),
        };

        Ok(generic_handle)
    }

    pub fn script(
        self,
        scene: &Scene,
        callback: impl FnOnce(&T) -> RisResult<()>,
    ) -> RisResult<()> {
        let ptr = scene.deref(self.handle.into())?;
        let aref = ptr.borrow();
        let script = aref.script.as_ref().unroll()?;
        let deref = std::ops::Deref::deref(script);


        panic!()
    }

    pub fn script_mut(
        self,
        scene: &ScriptEndData,
        callback: impl FnOnce(&mut T) -> RisResult<()>,
    ) -> RisResult<()> {
        // use arefcell instead box, then we can return aref and aref mut
        panic!()
    }
}

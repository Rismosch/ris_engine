use std::fmt::Debug;

use ris_error::RisResult;

use crate::gameloop::frame::Frame;
use crate::god_state::GodState;

use super::decl::GameObjectHandle;
use super::decl::ScriptComponentHandle;
use super::id::Component;
use super::scene::Scene;

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

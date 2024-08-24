use crate::ptr::ArefCell;
use crate::ptr::StrongPtr;
use crate::ptr::WeakPtr;

use ris_error::RisResult;

use crate::gameloop::frame::Frame;
use crate::god_state::GodState;

use super::id::EcsObject;
use super::id::IndexId;
use super::id::ScriptComponentHandle;
use super::id::GameObjectHandle;
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

pub trait Script {
    fn start(&mut self, data: ScriptStartData) -> RisResult<()>;
    fn update(&mut self, data: ScriptUpdateData) -> RisResult<()>;
    fn end(&mut self, data: ScriptEndData) -> RisResult<()>;
}

pub struct ScriptComponent {
    handle: ScriptComponentHandle,
    script: Option<Box<dyn Script>>,
}

impl ScriptComponent {
    pub fn new(
        handle: ScriptComponentHandle,
        script: Option<Box<dyn Script>>,
    ) -> Self {
        Self {
            handle,
            script,
        }
    }
}

impl EcsObject<IndexId> for ScriptComponent {
    fn handle(&self) -> ScriptComponentHandle {
        self.handle
    }

    fn is_alive(&self) -> bool {
        self.script.is_some()
    }
}

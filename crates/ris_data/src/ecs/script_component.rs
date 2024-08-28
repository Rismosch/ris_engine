use ris_error::RisResult;

use crate::gameloop::frame::Frame;
use crate::god_state::GodState;

use super::decl::ScriptComponentHandle;
use super::decl::GameObjectHandle;
use super::handle::GenericHandle;
use super::id::EcsObject;
use super::id::EcsTypeId;
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

pub trait Script : std::fmt::Debug {
    fn start(&mut self, data: ScriptStartData) -> RisResult<()>;
    fn update(&mut self, data: ScriptUpdateData) -> RisResult<()>;
    fn end(&mut self, data: ScriptEndData) -> RisResult<()>;
}

#[derive(Debug)]
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

impl EcsObject for ScriptComponent {
    fn handle(&self) -> GenericHandle<Self> {
        *self.handle
    }

    fn is_alive(&self) -> bool {
        self.script.is_some()
    }
}

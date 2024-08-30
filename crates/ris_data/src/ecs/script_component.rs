use ris_error::RisResult;

use crate::gameloop::frame::Frame;
use crate::god_state::GodState;

use super::decl::ScriptComponentHandle;
use super::decl::GameObjectHandle;
use super::handle::GenericHandle;
use super::id::EcsObject;
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
    script: Option<Box<dyn Script>>,
}

impl Default for ScriptComponent {
    fn default() -> Self {
        Self {
            script: None,
        }
    }
}


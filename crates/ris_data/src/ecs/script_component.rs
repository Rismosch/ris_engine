use crate::ptr::ArefCell;
use crate::ptr::StrongPtr;
use crate::ptr::WeakPtr;

use super::id::EcsObject;
use super::id::IndexId;
use super::id::ScriptComponentHandle;

pub struct ScriptComponent {
    // identification
    handle: ScriptComponentHandle,
    is_alive: bool,
}

impl ScriptComponent {
    pub fn new(handle: ScriptComponentHandle, is_alive: bool) -> Self {
        Self {
            handle,
            is_alive,
        }
    }
}

impl EcsObject<IndexId> for ScriptComponent {
    fn handle(&self) -> ScriptComponentHandle {
        self.handle
    }

    fn is_alive(&self) -> bool {
        self.is_alive
    }
}

use crate::ptr::ArefCell;
use crate::ptr::StrongPtr;
use crate::ptr::WeakPtr;

use super::id::ComponentHandle;
use super::id::IComponent;

pub struct Script {
    // identification
    handle: ComponentHandle,
    is_alive: bool,
}

impl IComponent for Script {
    fn type_id() -> usize {
        super::id::COMPONENT_TYPE_ID_SCRIPT
    }

    fn new(handle: ComponentHandle, is_alive: bool) -> Self {
        Self {
            handle,
            is_alive,
        }
    }

    fn handle(&self) -> ComponentHandle {
        self.handle
    }

    fn is_alive(&self) -> bool {
        self.is_alive
    }
}

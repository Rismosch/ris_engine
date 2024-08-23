use crate::ptr::ArefCell;
use crate::ptr::StrongPtr;
use crate::ptr::WeakPtr;

use super::id::EcsObject;
use super::id::IndexId;
use super::id::VisualMeshHandle;

pub struct VisualMesh {
    // identification
    handle: VisualMeshHandle,
    is_alive: bool,
}

impl VisualMesh {
    pub fn new(handle: VisualMeshHandle, is_alive: bool) -> Self {
        Self {
            handle,
            is_alive,
        }
    }
}

impl EcsObject<IndexId> for VisualMesh {
    fn handle(&self) -> VisualMeshHandle {
        self.handle
    }

    fn is_alive(&self) -> bool {
        self.is_alive
    }
}

use crate::ptr::ArefCell;
use crate::ptr::StrongPtr;
use crate::ptr::WeakPtr;

use super::id::EcsObject;
use super::id::IndexId;
use super::id::MeshComponentHandle;

pub struct MeshComponent {
    // identification
    handle: MeshComponentHandle,
    is_alive: bool,
}

impl MeshComponent {
    pub fn new(handle: MeshComponentHandle, is_alive: bool) -> Self {
        Self {
            handle,
            is_alive,
        }
    }
}

impl EcsObject<IndexId> for MeshComponent {
    fn handle(&self) -> MeshComponentHandle {
        self.handle
    }

    fn is_alive(&self) -> bool {
        self.is_alive
    }
}

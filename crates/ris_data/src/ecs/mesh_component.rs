use super::decl::MeshComponentHandle;
use super::id::EcsObject;
use super::handle::GenericHandle;

#[derive(Debug)]
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

impl EcsObject for MeshComponent {
    fn handle(&self) -> GenericHandle<Self> {
        *self.handle
    }

    fn is_alive(&self) -> bool {
        self.is_alive
    }
}

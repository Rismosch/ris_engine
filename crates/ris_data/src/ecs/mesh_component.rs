use super::id::EcsObject;
use super::id::EcsTypeId;
use super::handle::GenericHandle;
use super::handle::MeshComponentHandle;

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
    fn ecs_type_id() -> EcsTypeId {
        super::handle::ECS_TYPE_ID_MESH_COMPONENT
    }

    fn handle(&self) -> GenericHandle<Self> {
        *self.handle
    }

    fn is_alive(&self) -> bool {
        self.is_alive
    }
}

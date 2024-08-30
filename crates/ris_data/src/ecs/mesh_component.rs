use super::decl::MeshComponentHandle;
use super::id::EcsObject;
use super::handle::GenericHandle;

#[derive(Debug)]
pub struct MeshComponent {
}

impl Default for MeshComponent {
    fn default() -> Self {
        Self{}
    }
}

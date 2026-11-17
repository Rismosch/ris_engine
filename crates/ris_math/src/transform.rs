use crate::matrix::Mat4;
use crate::quaternion::Quat;
use crate::vector::Vec3;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Transform {
    position: Vec3,
    rotation: Quat,
    scale: Vec3,
    parent: u32,
}

impl Transform {
    pub fn model(self, ecs: impl AsRef<[Self]>) -> Mat4 {
        todo!();
    }
}

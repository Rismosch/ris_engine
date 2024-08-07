use ris_math::matrix::Mat4;
use ris_math::quaternion::Quat;
use ris_math::vector::Vec3;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GameObjectHandle {
    pub id: usize,
    pub generation: usize,
}

pub struct GameObject {
    id: usize,
    generation: usize,

    position: Vec3,
    rotation: Quat,
    scale: Vec3,
    model_mat: Mat4,
    model_mat_is_dirty: bool,

    parent: Option<GameObjectHandle>,
    children: Vec<GameObjectHandle>,
}

impl GameObject {
    pub fn get_local_position(&self) -> Vec3 {
        self.position
    }

    pub fn get_local_rotation(&self) -> Quat {
        self.rotation
    }

    pub fn get_local_scale(&self) -> Vec3 {
        self.scale
    }

    pub fn set_local_position(&mut self, position: Vec3) {
        self.position = position;
        self.model_mat_is_dirty = true;
    }

    pub fn set_local_rotation(&mut self, rotation: Quat) {
        self.rotation = rotation;
        self.model_mat_is_dirty = true;
    }

    pub fn set_local_scale(&mut self, scale: Vec3) {
        self.scale = scale;
        self.model_mat_is_dirty = true;
    }

    pub fn get_model_mat(&self) -> Mat4 {
        if self.model_mat_is_dirty {

        }

        self.model_mat
    }
}

use ris_math::matrix::Mat4;
use ris_math::quaternion::Quat;
use ris_math::vector::Vec3;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GameObjectHandle {
    pub id: usize,
    pub generation: usize,
}

pub struct GameObject {
    // identification
    id: usize,
    generation: usize,

    // local values
    is_visible: bool,
    position: Vec3,
    rotation: Quat,
    scale: f32,

    // hierarchy
    parent: Option<GameObjectHandle>,
    children: Vec<GameObjectHandle>,

    // cache
    cache_is_dirty: bool,
    is_visible_in_hierarchy: bool,
    world_position: Vec3,
    world_rotation: Quat,
    world_scale: f32,
    model: Mat4,
}

impl GameObject {

}

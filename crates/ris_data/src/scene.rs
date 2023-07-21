use ris_math::quaternion::Quaternion;
use ris_math::vector3::Vector3;

#[derive(Default, Clone)]
pub struct Scene {
    pub debug_x: i32,
    pub debug_y: i32,

    pub camera_position: Vector3,
    pub camera_rotation: Quaternion,
}

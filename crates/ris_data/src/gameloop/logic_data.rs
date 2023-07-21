use ris_math::quaternion::Quaternion;
use ris_math::vector3::Vector3;

use super::super::scene::Scene;

#[derive(Default, Clone)]
pub struct LogicData {
    pub camera_horizontal_angle: f32,
    pub camera_vertical_angle: f32,

    pub scene: Scene,
}

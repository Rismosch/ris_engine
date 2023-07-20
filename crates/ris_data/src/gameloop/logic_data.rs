use ris_math::quaternion::Quaternion;
use ris_math::vector3::Vector3;
use super::super::output::gpu_objects::Scene;

#[derive(Default, Clone)]
pub struct LogicData {
    pub camera_position: Vector3,
    pub camera_rotation: Quaternion,

    pub scene: Scene,
}

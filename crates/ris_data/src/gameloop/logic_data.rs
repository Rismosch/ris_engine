use ris_math::vector3::Quaternion;
use ris_math::vector3::Vector3;

#[derive(Default, Clone)]
pub struct LogicData {
    camera_position: Vector3,
    camera_rotation: Quaternion,
}

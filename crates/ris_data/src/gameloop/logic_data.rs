use ris_math::quaternion::Quaternion;
use ris_math::vector3::Vector3;

use crate::input::gamepad_data::GamepadData;
use crate::input::general_data::GeneralData;
use crate::input::keyboard_data::KeyboardData;
use crate::input::mouse_data::MouseData;

#[derive(Default, Clone)]
pub struct LogicData {
    // input
    //pub mouse: MouseData,
    //pub keyboard: KeyboardData,
    //pub gamepad: GamepadData,
    //pub general: GeneralData,

    // general
    //pub camera_position: Vector3,
    //pub camera_rotation: Quaternion,

    //pub camera_horizontal_angle: f32,
    //pub camera_vertical_angle: f32,

    //pub reload_shaders: bool,
    //pub window_size_changed: Option<(i32, i32)>,
}

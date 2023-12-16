use crate::input::gamepad_data::GamepadData;
use crate::input::general_data::GeneralData;
use crate::input::keyboard_data::KeyboardData;
use crate::input::mouse_data::MouseData;
use crate::scene::Scene;

#[derive(Default, Clone)]
pub struct LogicData {
    // input
    pub mouse: MouseData,
    pub keyboard: KeyboardData,
    pub gamepad: GamepadData,
    pub general: GeneralData,

    // general
    pub scene: Scene,

    pub camera_horizontal_angle: f32,
    pub camera_vertical_angle: f32,

    pub reload_shaders: bool,
    pub window_size_changed: Option<(i32, i32)>,
}

use crate::input::gamepad_data::GamepadData;
use crate::input::general_data::GeneralData;
use crate::input::keyboard_data::KeyboardData;
use crate::input::mouse_data::MouseData;

#[derive(Default, Clone)]
pub struct InputData {
    pub mouse: MouseData,
    pub keyboard: KeyboardData,
    pub gamepad: GamepadData,
    pub general: GeneralData,

    pub window_size_changed: Option<(i32, i32)>,
}

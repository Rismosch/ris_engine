use crate::input::{gamepad_data::GamepadData, keyboard_data::KeyboardData, mouse_data::MouseData, general_data::GeneralData};

#[derive(Default)]
pub struct InputData {
    pub mouse: MouseData,
    pub keyboard: KeyboardData,
    pub gamepad: GamepadData,
    pub general: GeneralData,
}

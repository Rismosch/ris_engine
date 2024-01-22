pub mod action;
pub mod buttons;
pub mod gamepad_data;
pub mod general_data;
pub mod keyboard_data;
pub mod keys;
pub mod mouse_data;
pub mod rebind_matrix;

use crate::input::gamepad_data::GamepadData;
use crate::input::general_data::GeneralData;
use crate::input::keyboard_data::KeyboardData;
use crate::input::mouse_data::MouseData;
use crate::input::rebind_matrix::RebindMatrix;

#[derive(Default, Clone)]
pub struct Input {
    pub mouse: MouseData,
    pub keyboard: KeyboardData,
    pub gamepad: GamepadData,
    pub general: GeneralData,
}

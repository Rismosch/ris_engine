use crate::input::{keyboard_data::KeyboardData, mouse_data::MouseData};

#[derive(Default)]
pub struct InputData {
    pub mouse: MouseData,
    pub keyboard: KeyboardData,
}

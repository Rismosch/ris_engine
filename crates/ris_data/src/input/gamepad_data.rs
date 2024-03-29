use crate::input::buttons::Buttons;
use crate::input::rebind_matrix::RebindMatrix;

#[derive(Clone)]
pub struct GamepadData {
    pub buttons: Buttons,
    pub axis: [i16; 6],

    pub deadzone_stick: i16,
    pub deadzone_trigger: i16,
    pub axis_button_threshhold: i16,

    pub rebind_matrix: RebindMatrix,
}

impl Default for GamepadData {
    fn default() -> Self {
        Self {
            buttons: Buttons::default(),
            axis: [0; 6],

            deadzone_stick: 10_000,
            deadzone_trigger: 1_000,
            axis_button_threshhold: i16::MAX / 2,
            rebind_matrix: RebindMatrix::default(),
        }
    }
}

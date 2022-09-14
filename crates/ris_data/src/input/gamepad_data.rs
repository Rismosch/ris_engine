use super::buttons::Buttons;

pub struct GamepadData {
    pub buttons: Buttons,
    pub axis: [i16; 6],

    pub deadzone_stick: i16,
    pub deadzone_trigger: i16,
    pub axis_button_threshhold: i16,
}

impl Default for GamepadData {
    fn default() -> Self {
        Self {
            buttons: Buttons::default(),
            axis: [0; 6],

            deadzone_stick: 10_000,
            deadzone_trigger: 1_000,
            axis_button_threshhold: i16::MAX / 2,
        }
    }
}

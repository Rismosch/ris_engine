pub static ALL_BUTTONS: [sdl2::controller::Button; 15] = [
    sdl2::controller::Button::A,
    sdl2::controller::Button::B,
    sdl2::controller::Button::X,
    sdl2::controller::Button::Y,
    sdl2::controller::Button::Back,
    sdl2::controller::Button::Guide,
    sdl2::controller::Button::Start,
    sdl2::controller::Button::LeftStick,
    sdl2::controller::Button::RightStick,
    sdl2::controller::Button::LeftShoulder,
    sdl2::controller::Button::RightShoulder,
    sdl2::controller::Button::DPadUp,
    sdl2::controller::Button::DPadDown,
    sdl2::controller::Button::DPadLeft,
    sdl2::controller::Button::DPadRight,
];

pub fn get_button_index(axis: sdl2::controller::Axis, value_is_negative: bool) -> usize {
    match axis {
        sdl2::controller::Axis::LeftX => if value_is_negative {
            15
        } else {
            16
        },
        sdl2::controller::Axis::LeftY => if value_is_negative {
            17
        } else {
            18
        },
        sdl2::controller::Axis::RightX => if value_is_negative {
            19
        } else {
            20
        },
        sdl2::controller::Axis::RightY => if value_is_negative {
            21
        } else {
            22
        },
        sdl2::controller::Axis::TriggerLeft => 23,
        sdl2::controller::Axis::TriggerRight => 24,
    }
}
use super::buttons::Buttons;

#[derive(Default)]
pub struct MouseData {
    pub buttons: Buttons,
    pub x: i32,
    pub y: i32,
    pub xrel: i32,
    pub yrel: i32,
    pub wheel_xrel: i32,
    pub wheel_yrel: i32,
}

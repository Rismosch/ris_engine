use crate::input::buttons::Buttons;
use crate::input::rebind_matrix::RebindMatrix;

#[derive(Default, Clone)]
pub struct MouseData {
    pub buttons: Buttons,
    pub x: i32,
    pub y: i32,
    pub xrel: i32,
    pub yrel: i32,
    pub wheel_xrel: i32,
    pub wheel_yrel: i32,

    pub rebind_matrix: RebindMatrix,
}

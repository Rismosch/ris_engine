use sdl2::keyboard::Scancode;

use super::buttons::Buttons;

pub struct KeyboardData {
    pub buttons: Buttons,
    pub keymask: [Scancode; 32],
}

impl Default for KeyboardData {
    fn default() -> Self {
        Self {
            buttons: Buttons::default(),
            keymask: [Scancode::A; 32],
        }
    }
}

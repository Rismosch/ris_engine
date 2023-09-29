use sdl2::keyboard::Scancode;

use crate::input::buttons::Buttons;
use crate::input::keys::Keys;

#[derive(Clone)]
pub struct KeyboardData {
    pub buttons: Buttons,
    pub keymask: [Scancode; 32],
    pub keys: Keys,
}

impl KeyboardData {
    pub fn new(keymask: [Scancode; 32]) -> Self {
        Self {
            buttons: Buttons::default(),
            keymask,
            keys: Keys::default(),
        }
    }
}

impl Default for KeyboardData {
    fn default() -> Self {
        Self::new([Scancode::A; 32])
    }
}

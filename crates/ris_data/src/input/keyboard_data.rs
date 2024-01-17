use sdl2::keyboard::Mod;
use sdl2::keyboard::Scancode;

use crate::input::buttons::Buttons;
use crate::input::keys::Keys;

#[derive(Clone)]
pub struct KeyboardData {
    pub buttons: Buttons,
    pub keymask: [Scancode; 32],
    pub keys: Keys,
    pub mod_state: Mod,
    pub text_input: Vec<String>,
}

impl KeyboardData {
    pub fn new(keymask: [Scancode; 32]) -> Self {
        Self {
            buttons: Buttons::default(),
            keymask,
            keys: Keys::default(),
            mod_state: Mod::NOMOD,
            text_input: Vec::new(),
        }
    }
}

impl Default for KeyboardData {
    fn default() -> Self {
        Self::new([Scancode::A; 32])
    }
}

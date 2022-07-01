use sdl2::keyboard::Scancode;

use crate::buttons::{Buttons, IButtons};

pub struct Keyboard {
    buttons: Buttons,

    keymask: [Scancode; 32],
}

pub trait IKeyboard {
    fn buttons(&self) -> &Buttons;

    fn keymask(&self) -> &[Scancode; 32];
    fn set_keymask(&mut self, keymask: [Scancode; 32]);
}

impl Default for Keyboard {
    fn default() -> Self {
        Keyboard {
            buttons: Buttons::default(),
            keymask: [Scancode::A; 32],
        }
    }
}

impl IKeyboard for Keyboard {
    fn buttons(&self) -> &Buttons {
        &self.buttons
    }

    fn keymask(&self) -> &[Scancode; 32] {
        &self.keymask
    }
    fn set_keymask(&mut self, keymask: [Scancode; 32]) {
        self.keymask[..32].copy_from_slice(&keymask[..32]);
    }
}

impl Keyboard {
    pub fn update_state(&mut self, keyboard_state: sdl2::keyboard::KeyboardState) {
        let mut new_state = 0;

        for (scancode, value) in keyboard_state.scancodes() {
            if !value {
                continue;
            }

            for i in 0..32 {
                if self.keymask[i] == scancode {
                    new_state |= 1 << i;
                }
            }
        }

        self.buttons.update(&new_state);
    }
}

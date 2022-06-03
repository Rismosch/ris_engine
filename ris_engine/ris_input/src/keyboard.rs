use sdl2::keyboard::Scancode;

use crate::gate::{Gate, IGate};

pub struct Keyboard {
    gate: Gate,

    keymask: [Scancode; 32],
}

pub trait IKeyboard {
    fn gate(&self) -> &Gate;

    fn keymask(&self) -> &[Scancode; 32];
    fn set_keymask(&mut self, key_mask: &[Scancode; 32]);

    fn update_state(&mut self, keyboard_state: sdl2::keyboard::KeyboardState);
}

impl Default for Keyboard {
    fn default() -> Self {
        Keyboard {
            gate: Gate::default(),
            keymask: [Scancode::A; 32],
        }
    }
}

impl IKeyboard for Keyboard {
    fn gate(&self) -> &Gate {
        &self.gate
    }

    fn keymask(&self) -> &[Scancode; 32] {
        &self.keymask
    }
    fn set_keymask(&mut self, key_mask: &[Scancode; 32]) {
        self.keymask[..32].copy_from_slice(&key_mask[..32]);
    }

    fn update_state(&mut self, keyboard_state: sdl2::keyboard::KeyboardState) {
        let mut new_state = 0;

        for (scancode, value) in keyboard_state.scancodes() {
            let mut button_mask = 0;

            for i in 0..32 {
                if self.keymask[i] == scancode {
                    button_mask |= 1 << i;
                }
            }

            if value {
                new_state |= button_mask;
            } else {
                new_state &= !button_mask;
            }
        }

        self.gate.update(new_state);
    }
}

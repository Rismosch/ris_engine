use std::time::Instant;

use sdl2::keyboard::Scancode;

use super::buttons::Buttons;

pub struct KeyboardData {
    pub buttons: Buttons,
    pub keymask: [Scancode; 32],

    pub crash_timestamp: Instant,
    pub restart_timestamp: Instant,
}

impl Default for KeyboardData {
    fn default() -> Self {
        Self {
            buttons: Buttons::default(),
            keymask: [Scancode::A; 32],
            crash_timestamp: Instant::now(),
            restart_timestamp: Instant::now(),
        }
    }
}

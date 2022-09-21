use std::time::Instant;

use ris_data::{gameloop::gameloop_state::GameloopState, input::keyboard_data::KeyboardData};
use sdl2::keyboard::Scancode;

pub fn update_keyboard(
    new_keyboard_data: &mut KeyboardData,
    old_keyboard_data: &KeyboardData,
    keyboard_state: sdl2::keyboard::KeyboardState,
) -> GameloopState {
    let mut new_state = 0;
    let old_state = old_keyboard_data.buttons.hold();

    for (scancode, value) in keyboard_state.scancodes() {
        let should_crash = manual_crash(new_keyboard_data, old_keyboard_data, scancode, value);

        if !matches!(should_crash, GameloopState::WantsToContinue) {
            return should_crash;
        }

        if !value {
            continue;
        }

        for i in 0..32 {
            if new_keyboard_data.keymask[i] == scancode {
                new_state |= 1 << i;
            }
        }
    }

    new_keyboard_data.buttons.set(&new_state, &old_state);

    GameloopState::WantsToContinue
}

fn manual_crash(
    new_keyboard_data: &mut KeyboardData,
    old_keyboard_data: &KeyboardData,
    scancode: Scancode,
    value: bool,
) -> GameloopState {
    const TIMEOUT: u64 = 5;

    if matches!(scancode, Scancode::F12) {
        if value {
            new_keyboard_data.crash_timestamp = old_keyboard_data.crash_timestamp;
        } else {
            new_keyboard_data.crash_timestamp = Instant::now();
        }

        let duration = Instant::now() - old_keyboard_data.crash_timestamp;
        let seconds = duration.as_secs();

        if seconds >= TIMEOUT {
            ris_log::fatal!("manual crash reqeusted");
            return GameloopState::Error(String::from("manual crash"));
        }
    }

    if matches!(scancode, Scancode::F10) {
        if value {
            new_keyboard_data.restart_timestamp = old_keyboard_data.restart_timestamp;
        } else {
            new_keyboard_data.restart_timestamp = Instant::now();
        }

        let duration = Instant::now() - old_keyboard_data.restart_timestamp;
        let seconds = duration.as_secs();

        if seconds >= TIMEOUT {
            ris_log::fatal!("restart reqeusted");
            return GameloopState::WantsToRestart;
        }
    }

    GameloopState::WantsToContinue
}

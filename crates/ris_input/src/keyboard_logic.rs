use ris_data::{input::keyboard_data::KeyboardData, gameloop::gameloop_state::GameloopState};
use sdl2::keyboard::Scancode;

pub fn update_keyboard(
    new_keyboard_data: &mut KeyboardData,
    old_keyboard_data: &KeyboardData,
    keyboard_state: sdl2::keyboard::KeyboardState,
) -> GameloopState {
    let mut new_state = 0;
    let old_state = old_keyboard_data.buttons.hold();

    for (scancode, value) in keyboard_state.scancodes() {
        let should_crash = manual_crash(scancode, value);
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

    new_keyboard_data.buttons.update(&new_state, &old_state);

    GameloopState::WantsToContinue
}

fn manual_crash(scancode: Scancode, value: bool) -> GameloopState {
    if matches!(scancode, Scancode::F12) {

    }
    
    if matches!(scancode, Scancode::F11) {
        
    }

    GameloopState::WantsToContinue
}

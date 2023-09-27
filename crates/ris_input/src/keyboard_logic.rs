use ris_data::gameloop::gameloop_state::GameloopState;
use ris_data::input::keyboard_data::KeyboardData;

pub fn update_keyboard(
    new_keyboard_data: &mut KeyboardData,
    old_keyboard_data: &KeyboardData,
    keyboard_state: sdl2::keyboard::KeyboardState,
) -> GameloopState {
    let mut new_state = 0;
    let old_state = old_keyboard_data.buttons.hold();

    for (scancode, value) in keyboard_state.scancodes() {
        if !value {
            continue;
        }

        for i in 0..32 {
            if new_keyboard_data.keymask[i] == scancode {
                new_state |= 1 << i;
            }
        }
    }

    new_keyboard_data.buttons.set(new_state, old_state);

    GameloopState::WantsToContinue
}

//fn manual_crash(
//    new_keyboard_data: &mut KeyboardData,
//    old_keyboard_data: &KeyboardData,
//    scancode: Scancode,
//) -> GameloopState {
//    const TIMEOUT: u64 = 5;
//
//    match scancode {
//        Scancode::F3 => {
//            new_keyboard_data.crash_timestamp = old_keyboard_data.crash_timestamp;
//
//            let duration = Instant::now() - old_keyboard_data.crash_timestamp;
//            let seconds = duration.as_secs();
//
//            if seconds >= TIMEOUT {
//                ris_log::fatal!("manual crash reqeusted");
//                return GameloopState::Error(ris_util::new_err!("manual crash"));
//            }
//        }
//        Scancode::F4 => {
//            new_keyboard_data.restart_timestamp = old_keyboard_data.restart_timestamp;
//
//            let duration = Instant::now() - old_keyboard_data.restart_timestamp;
//            let seconds = duration.as_secs();
//
//            if seconds >= TIMEOUT {
//                ris_log::fatal!("restart reqeusted");
//                return GameloopState::WantsToRestart;
//            }
//        }
//        _ => (),
//    }
//
//    GameloopState::WantsToContinue
//}
//
//fn reset_manual_crash(new_keyboard_data: &mut KeyboardData, scancode: Scancode) {
//    match scancode {
//        Scancode::F12 => new_keyboard_data.crash_timestamp = Instant::now(),
//        Scancode::F10 => new_keyboard_data.restart_timestamp = Instant::now(),
//        _ => (),
//    }
//}

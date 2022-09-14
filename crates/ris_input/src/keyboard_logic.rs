use ris_data::input::keyboard_data::KeyboardData;

pub fn update_keyboard(
    new_keyboard_data: &mut KeyboardData,
    old_keyboard_data: &KeyboardData,
    keyboard_state: sdl2::keyboard::KeyboardState,
) {
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

    new_keyboard_data.buttons.update(&new_state, &old_state);
}

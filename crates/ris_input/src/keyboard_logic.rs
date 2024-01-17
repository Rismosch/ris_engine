use sdl2::event::Event;

use ris_data::input::keyboard_data::KeyboardData;

pub fn pre_events(keyboard_data: &mut KeyboardData) {
    keyboard_data.text_input.clear();
}

pub fn handle_event(keyboard_data: &mut KeyboardData, event: &Event) {
    if let Event::TextInput { text, .. } = event {
        keyboard_data.text_input.push(text.to_owned());
    }
}

pub fn post_events(
    new_keyboard_data: &mut KeyboardData,
    old_keyboard_data: &KeyboardData,
    keyboard_state: sdl2::keyboard::KeyboardState,
    mod_state: sdl2::keyboard::Mod,
) {
    let old_key_state = old_keyboard_data.keys.hold();
    new_keyboard_data.keys.set_old_and_clear(old_key_state);

    let mut new_button_state = 0;
    let old_button_state = old_keyboard_data.buttons.hold();

    for (scancode, value) in keyboard_state.scancodes() {
        if !value {
            continue;
        }

        new_keyboard_data.keys.set(scancode);

        for i in 0..32 {
            if new_keyboard_data.keymask[i] == scancode {
                new_button_state |= 1 << i;
            }
        }
    }

    new_keyboard_data
        .buttons
        .set(new_button_state, old_button_state);

    new_keyboard_data.mod_state = mod_state;
}

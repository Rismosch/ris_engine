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
    keyboard_data: &mut KeyboardData,
    keyboard_state: sdl2::keyboard::KeyboardState,
    mod_state: sdl2::keyboard::Mod,
) {
    keyboard_data.keys.clear();

    let mut new_button_state = 0;

    for (scancode, value) in keyboard_state.scancodes() {
        if !value {
            continue;
        }

        keyboard_data.keys.set(scancode);

        for i in 0..32 {
            if keyboard_data.keymask[i] == scancode {
                new_button_state |= 1 << i;
            }
        }
    }

    keyboard_data.buttons.update(new_button_state);
    keyboard_data.mod_state = mod_state;
}

use sdl2_sys::SDL_Event;
use sdl2_sys::SDL_EventType;

use ris_data::input::keyboard_data::KeyboardData;

pub fn pre_events(keyboard_data: &mut KeyboardData) {
    keyboard_data.text_input.clear();
}

pub unsafe fn handle_event(keyboard_data: &mut KeyboardData, event: &SDL_Event) {
    if event.type_ == SDL_EventType::SDL_TEXTINPUT as u32 {
        let text_raw = event.text.text;
        let text_bytes = text_raw
            .iter()
            .take_while(|&&x| x != 0)
            .map(|&x| x as u8)
            .collect::<Vec<_>>();
        
        if let Ok(text) = String::from_utf8(text_bytes) {
            keyboard_data.text_input.push(text);
        }
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

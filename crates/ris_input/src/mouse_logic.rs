use sdl2_sys::SDL_Event;
use sdl2_sys::SDL_EventType;

use ris_data::input::mouse_data::MouseData;

pub fn pre_events(mouse_data: &mut MouseData) {
    mouse_data.xrel = 0;
    mouse_data.yrel = 0;
    mouse_data.wheel_xrel = 0;
    mouse_data.wheel_yrel = 0;
}

pub unsafe fn handle_event(mouse_data: &mut MouseData, event: &SDL_Event) {
    if event.type_ == SDL_EventType::SDL_MOUSEMOTION as u32 {
        mouse_data.x = event.motion.x;
        mouse_data.y = event.motion.y;
        mouse_data.xrel += event.motion.xrel;
        mouse_data.yrel += event.motion.yrel;
    }

    if event.type_ == SDL_EventType::SDL_MOUSEWHEEL as u32 {
        mouse_data.wheel_xrel += event.wheel.x;
        mouse_data.wheel_yrel += event.wheel.y;
    }
}

pub fn post_events(mouse_data: &mut MouseData, mouse_state: sdl2::mouse::MouseState) {
    let new_state = mouse_state.to_sdl_state();
    mouse_data.buttons.update(new_state);
}

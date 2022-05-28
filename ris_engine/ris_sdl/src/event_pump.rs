use sdl2::EventPump;

use crate::context::context;

static mut EVENT_PUMP: Option<Box<sdl2::EventPump>> = None;

/// # Safety
/// Should only be called by the main thread.
/// This method modifies global static variables, and thus is inherently unsafe.
pub unsafe fn init() -> Result<(), Box<dyn std::error::Error>> {
    let sdl_context = context();

    let event_pump = sdl_context.event_pump()?;

    EVENT_PUMP = Some(Box::new(event_pump));

    Ok(())
}

// pub fn poll_iter() -> sdl2::event::EventPollIterator<'static> {
//     let event_pump = get_event_pump();
//     event_pump.as_mut().poll_iter()
// }

pub fn keyboard_state() -> sdl2::keyboard::KeyboardState<'static> {
    let event_pump = get_event_pump();
    event_pump.keyboard_state()
}

pub fn mouse_state() -> sdl2::mouse::MouseState {
    let event_pump = get_event_pump();
    event_pump.mouse_state()
}

fn get_event_pump() -> &'static mut Box<EventPump> {
    unsafe {
        match &mut EVENT_PUMP {
            Some(event_pump) => event_pump,
            None => panic!("sdl is not initialized"),
        }
    }
}

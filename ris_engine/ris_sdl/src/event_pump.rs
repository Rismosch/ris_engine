use sdl2::EventPump;
use sdl2::event::Event;

use crate::context::context;

#[derive(Default)]
struct EventState
{
    quit_was_called: bool,
}

static mut EVENT_PUMP: Option<sdl2::EventPump> = None;
static mut EVENT_STATE: Option<EventState> = None;

/// # Safety
/// Should only be called by the main thread.
/// This method modifies global static variables, and thus is inherently unsafe.
pub unsafe fn init() -> Result<(), Box<dyn std::error::Error>> {
    let sdl_context = context();

    let event_pump = sdl_context.event_pump()?;

    EVENT_PUMP = Some(event_pump);
    EVENT_STATE = Some(EventState::default());

    Ok(())
}

pub fn poll_all_events() {
    let event_pump = get_event_pump();
    let event_state = get_event_state();

    for event in event_pump.poll_iter() {
        if let Event::Quit { .. } = event {
            event_state.quit_was_called = true;
        };
    }
}

pub fn keyboard_state() -> sdl2::keyboard::KeyboardState<'static> {
    let event_pump = get_event_pump();
    event_pump.keyboard_state()
}

pub fn mouse_state() -> sdl2::mouse::MouseState {
    let event_pump = get_event_pump();
    event_pump.mouse_state()
}

pub fn quit_was_called() -> bool {
    get_event_state().quit_was_called
}

fn get_event_pump() -> &'static mut EventPump {
    unsafe {
        match &mut EVENT_PUMP {
            Some(event_pump) => event_pump,
            None => panic!("eventpump is not initialized"),
        }
    }
}

fn get_event_state() -> &'static mut EventState {
    unsafe {
        match &mut EVENT_STATE {
            Some(event_state) => event_state,
            None => panic!("eventpump is not initialized"),
        }
    }
}
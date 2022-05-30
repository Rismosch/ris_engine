use sdl2::event::Event;
use sdl2::EventPump;

use crate::context::context;

static mut EVENT_PUMP: Option<sdl2::EventPump> = None;

/// # Safety
/// Should only be called by the main thread.
/// This method modifies global static variables, and thus is inherently unsafe.
pub unsafe fn init() -> Result<(), Box<dyn std::error::Error>> {
    let sdl_context = context();

    let event_pump = sdl_context.event_pump()?;

    EVENT_PUMP = Some(event_pump);

    Ok(())
}

pub fn poll_all_events() {
    let event_pump = get_event_pump();

    let mut mouse_events = Vec::new();

    for event in event_pump.poll_iter() {
        println!("{:?}",event);

        if let Event::Quit { .. } = event {
            // event_state.quit_was_called = true;
        };

        handle_mouse_events(event, &mut mouse_events);
    }

    println!("{} {:?}", mouse_events.len(), mouse_events);
}

fn handle_mouse_events(event: Event, mouse_events: &mut Vec<Event>)
{
    match event {
        Event::MouseMotion { .. }
        | Event::MouseButtonDown { .. }
        | Event::MouseButtonUp { .. }
        | Event::MouseWheel { .. } => mouse_events.push(event),
        _ => (),
    };
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
    false
}

fn get_event_pump() -> &'static mut EventPump {
    unsafe {
        match &mut EVENT_PUMP {
            Some(event_pump) => event_pump,
            None => panic!("eventpump is not initialized"),
        }
    }
}

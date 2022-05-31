use std::rc::Weak;

use sdl2::event::Event;
use sdl2::EventPump as Sdl2Pump;
use sdl2::Sdl;

pub struct EventPump {
    wants_to_quit: bool,
    sdl2_pump: Sdl2Pump,

    mouse_observer: Option<Weak<dyn IEventObserver>>,
}

pub trait IEventPump {
    fn wants_to_quit(&self) -> bool;

    fn pump(&mut self);

    fn subscribe_mouse(&mut self, observer: Weak<dyn IEventObserver>);

    fn keyboard_state(&self) -> sdl2::keyboard::KeyboardState;
    fn mouse_state(&self) -> sdl2::mouse::MouseState;
}

pub trait IEventObserver {
    fn update(&self, events: &Vec<Event>);
}

impl EventPump {
    pub fn new(sdl_context: &Sdl) -> Result<EventPump, Box<dyn std::error::Error>> {
        let sdl2_pump = sdl_context.event_pump()?;
        let wants_to_quit = false;

        let event_pump = EventPump {
            wants_to_quit,
            sdl2_pump,
            mouse_observer: None,
        };
        Ok(event_pump)
    }
}

impl IEventPump for EventPump {
    fn wants_to_quit(&self) -> bool {
        self.wants_to_quit
    }

    fn pump(&mut self) {
        let mut mouse_events = Vec::new();

        for event in self.sdl2_pump.poll_iter() {
            // println!("{:?}", event);

            if let Event::Quit { .. } = event {
                self.wants_to_quit = true;
            };

            match_mouse_event(event, &mut mouse_events);
        }

        if let Some(mouse_observer) = &self.mouse_observer {
            if let Some(mouse_observer) = mouse_observer.upgrade(){
                mouse_observer.update(&mouse_events);
            }
        }
    }

    fn subscribe_mouse(&mut self, observer: Weak<dyn IEventObserver>) {
        self.mouse_observer = Some(observer);
    }

    fn keyboard_state(&self) -> sdl2::keyboard::KeyboardState {
        self.sdl2_pump.keyboard_state()
    }

    fn mouse_state(&self) -> sdl2::mouse::MouseState {
        self.sdl2_pump.mouse_state()
    }
}

fn match_mouse_event(event: Event, mouse_events: &mut Vec<Event>) {
    match event {
        Event::MouseMotion { .. }
        | Event::MouseButtonDown { .. }
        | Event::MouseButtonUp { .. }
        | Event::MouseWheel { .. } => mouse_events.push(event),
        _ => (),
    };
}

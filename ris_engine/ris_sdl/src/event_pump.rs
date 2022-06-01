use std::rc::Weak;

use sdl2::event::Event;
use sdl2::EventPump as Sdl2Pump;
use sdl2::Sdl;

pub struct EventPump {
    wants_to_quit: bool,
    sdl2_pump: Sdl2Pump,

    event_observers: Vec<Weak<dyn IEventObserver>>,
}

pub trait IEventPump {
    fn wants_to_quit(&self) -> bool;

    fn pump(&mut self);

    fn subscribe(&mut self, observer: Weak<dyn IEventObserver>);

    fn keyboard_state(&self) -> sdl2::keyboard::KeyboardState;
    fn mouse_state(&self) -> sdl2::mouse::MouseState;
}

pub trait IEventObserver {
    fn pre_update(&self);
    fn update(&self, events: &Event);
}

impl EventPump {
    pub fn new(sdl_context: &Sdl) -> Result<EventPump, Box<dyn std::error::Error>> {
        let wants_to_quit = false;
        let sdl2_pump = sdl_context.event_pump()?;
        let event_observers = Vec::new();

        let event_pump = EventPump {
            wants_to_quit,
            sdl2_pump,
            event_observers,
        };
        Ok(event_pump)
    }
}

impl IEventPump for EventPump {
    fn wants_to_quit(&self) -> bool {
        self.wants_to_quit
    }

    fn pump(&mut self) {
        for event_observer in &self.event_observers {
            if let Some(event_observer) = event_observer.upgrade() {
                event_observer.pre_update();
            }
        }

        for event in self.sdl2_pump.poll_iter() {
            // println!("{:?}", event);

            if let Event::Quit { .. } = event {
                self.wants_to_quit = true;
            };

            for event_observer in &self.event_observers {
                if let Some(event_observer) = event_observer.upgrade() {
                    event_observer.update(&event);
                }
            }
        }
    }

    fn subscribe(&mut self, observer: Weak<dyn IEventObserver>) {
        self.event_observers.push(observer);
    }

    fn keyboard_state(&self) -> sdl2::keyboard::KeyboardState {
        self.sdl2_pump.keyboard_state()
    }

    fn mouse_state(&self) -> sdl2::mouse::MouseState {
        self.sdl2_pump.mouse_state()
    }
}

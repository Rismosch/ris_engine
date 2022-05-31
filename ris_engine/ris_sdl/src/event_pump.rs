use sdl2::event::Event;
use sdl2::EventPump as Sdl2Pump;
use sdl2::Sdl;

pub struct EventPump {
    sdl2_pump: Sdl2Pump,
    pub wants_to_quit: bool,
}

impl EventPump {
    pub fn new(sdl_context: Sdl) -> Result<EventPump, Box<dyn std::error::Error>> {
        let sdl2_pump = sdl_context.event_pump()?;
        let wants_to_quit = false;

        let event_pump = EventPump {
            sdl2_pump,
            wants_to_quit,
        };
        Ok(event_pump)
    }

    pub fn pump(&mut self) {
        let mut mouse_events = Vec::new();

        for event in self.sdl2_pump.poll_iter() {
            println!("{:?}", event);

            if let Event::Quit { .. } = event {
                self.wants_to_quit = true;
            };

            match_mouse_event(event, &mut mouse_events);
        }
    }

    pub fn keyboard_state(&self) -> sdl2::keyboard::KeyboardState {
        self.sdl2_pump.keyboard_state()
    }

    pub fn mouse_state(&self) -> sdl2::mouse::MouseState {
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

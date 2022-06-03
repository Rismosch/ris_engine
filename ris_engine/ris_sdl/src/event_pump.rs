use sdl2::event::Event;
use sdl2::EventPump as Sdl2Pump;
use sdl2::Sdl;

use crate::event_observer::IMouseObserver;

pub struct EventPump<TMouseObserver: IMouseObserver> {
    wants_to_quit: bool,
    sdl2_pump: Sdl2Pump,

    mouse_observer: TMouseObserver,
    // keyboard_observer: IKeyboardObserver,
    // game_controller_observer: IGameControllerObserver,
}

pub trait IEventPump {
    fn wants_to_quit(&self) -> bool;

    fn pump(&mut self);

    fn keyboard_state(&self) -> sdl2::keyboard::KeyboardState;
    fn mouse_state(&self) -> sdl2::mouse::MouseState;
}

impl<TMouseObserver: IMouseObserver> EventPump<TMouseObserver> {
    pub fn new(
        sdl_context: &Sdl,
        mouse_observer: TMouseObserver,
    ) -> Result<EventPump<TMouseObserver>, Box<dyn std::error::Error>> {
        let wants_to_quit = false;
        let sdl2_pump = sdl_context.event_pump()?;

        let event_pump = EventPump {
            wants_to_quit,
            sdl2_pump,
            mouse_observer,
        };
        Ok(event_pump)
    }
}

impl<TMouseObserver: IMouseObserver> IEventPump for EventPump<TMouseObserver> {
    fn wants_to_quit(&self) -> bool {
        self.wants_to_quit
    }

    fn pump(&mut self) {
        self.mouse_observer.pre_update();

        for event in self.sdl2_pump.poll_iter() {
            // println!("{:?}", event);

            if let Event::Quit { .. } = event {
                self.wants_to_quit = true;
            };

            self.mouse_observer.update(&event);
        }

        self.mouse_observer
            .update_state(self.sdl2_pump.mouse_state());
        self.mouse_observer.post_update();
    }

    fn keyboard_state(&self) -> sdl2::keyboard::KeyboardState {
        self.sdl2_pump.keyboard_state()
    }

    fn mouse_state(&self) -> sdl2::mouse::MouseState {
        self.sdl2_pump.mouse_state()
    }
}

use std::{
    thread,
    time::{Duration, Instant},
};

use ris_data::frame_buffer::FrameBuffer;
use ris_input::{
    buttons::IButtons,
    gamepad::{Gamepad, IGamepad},
    keyboard::{IKeyboard, Keyboard},
    mouse::{IMouse, Mouse},
};
use ris_sdl::video::Video;

use sdl2::EventPump;
use sdl2::{event::Event, keyboard::Scancode};

pub struct Engine {
    _video: Video,
    event_pump: EventPump,
    frame_buffer: FrameBuffer,
    mouse: Mouse,
    keyboard: Keyboard,
    gamepad: Gamepad,
}

impl Engine {
    pub fn new() -> Result<Engine, String> {
        let sdl_context = sdl2::init()?;

        let _video = Video::new(&sdl_context)?;
        let event_pump = sdl_context.event_pump()?;

        let frame_buffer = FrameBuffer::new(4);

        let mouse = Mouse::default();
        let mut keyboard = Keyboard::default();
        let gamepad = Gamepad::new(&sdl_context)?;

        let mut keymask = [Scancode::Space; 32];
        keymask[0] = Scancode::W;
        keymask[1] = Scancode::A;
        keymask[2] = Scancode::S;
        keymask[3] = Scancode::D;

        keyboard.set_keymask(&keymask);

        let engine = Engine {
            _video,
            event_pump,
            frame_buffer,
            mouse,
            keyboard,
            gamepad,
        };

        Ok(engine)
    }

    pub fn run(&mut self) -> Result<(), String> {
        loop {
            let now = Instant::now();

            let pump_wants_to_quit = self.pump_events();
            let game_wants_to_quit = self.game_logic();

            let delta = now.elapsed();

            self.frame_buffer.add(delta);

            if pump_wants_to_quit || game_wants_to_quit {
                break;
            }
        }

        Ok(())
    }

    fn pump_events(&mut self) -> bool {
        self.mouse.pre_update();
        self.gamepad.pre_update();

        for event in self.event_pump.poll_iter() {
            // println!("{:?}", event);

            if let Event::Quit { .. } = event {
                return true;
            };

            self.mouse.update(&event);
            self.gamepad.update(&event);
        }

        self.mouse.update_state(self.event_pump.mouse_state());
        self.keyboard.update_state(self.event_pump.keyboard_state());
        self.gamepad.update_state();

        false
    }

    fn game_logic(&mut self) -> bool {
        thread::sleep(Duration::from_millis(200));
        // println!("{}",self.frame_buffer.fps());

        // println!("{:#034b}", self.keyboard.buttons().hold());
        // println!("{:#034b}",self.mouse.buttons().hold());

        false
    }
}

use std::{
    thread,
    time::{Duration, Instant},
};

use ris_data::frame_buffer::FrameBuffer;
use ris_input::input::{IInput, Input};
use ris_sdl::video::Video;

use sdl2::event::Event;
use sdl2::EventPump;

pub struct Engine {
    _video: Video,
    event_pump: EventPump,
    frame_buffer: FrameBuffer,
    input: Input,
}

impl Engine {
    pub fn new() -> Result<Engine, String> {
        let sdl_context = sdl2::init()?;

        let _video = Video::new(&sdl_context)?;
        let event_pump = sdl_context.event_pump()?;

        let frame_buffer = FrameBuffer::new(4);

        let input = Input::new(&sdl_context)?;

        let engine = Engine {
            _video,
            event_pump,
            frame_buffer,
            input,
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
        // self.mouse.pre_update();
        self.input.pre_update();

        for event in self.event_pump.poll_iter() {
            // println!("{:?}", event);

            if let Event::Quit { .. } = event {
                return true;
            };

            // self.mouse.update(&event);
            self.input.update(&event);
        }

        // self.mouse.update_state(self.event_pump.mouse_state());
        // self.keyboard.update_state(self.event_pump.keyboard_state());
        // self.gamepad.update_state();
        self.input.post_update(&self.event_pump);

        false
    }

    fn game_logic(&mut self) -> bool {
        thread::sleep(Duration::from_millis(50));
        println!("{}", self.frame_buffer.fps());

        // println!("{:#034b}",self.input.mouse().buttons().hold());
        // println!("{:#034b}", self.input.keyboard().buttons().hold());
        // println!("{:#034b}",self.input.gamepad().buttons().hold());
        // let axis = self.input.gamepad().axis();
        // println!("{}\t{}\t{}\t{}\t{}\t{}", axis[0], axis[1], axis[2], axis[3], axis[4], axis[5]);

        false
    }
}

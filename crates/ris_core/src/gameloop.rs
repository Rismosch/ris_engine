use std::time::Instant;

use ris_input::{buttons::IButtons, input::IInput};

use sdl2::event::Event;
use sdl2::EventPump;

use crate::god_object::GodObject;

pub enum GameloopState {
    Running,
    WantsToQuit,
    Error(String),
}

pub fn run_one_frame(god_object: &mut GodObject) -> GameloopState {
    let now = Instant::now();

    let pump_wants_to_quit = pump_events(&mut god_object.input, &mut god_object.event_pump);
    let game_wants_to_quit = game_logic(&god_object.input);

    let delta = now.elapsed();

    god_object.frame_buffer.add(delta);

    if pump_wants_to_quit || game_wants_to_quit {
        GameloopState::WantsToQuit
    } else {
        GameloopState::Running
    }
}

fn pump_events<TInput: IInput>(input: &mut TInput, event_pump: &mut EventPump) -> bool {
    input.pre_update();

    for event in event_pump.poll_iter() {
        // ris_log::trace!("{:?}", event);

        if let Event::Quit { .. } = event {
            return true;
        };

        input.update(&event);
    }

    input.post_update(event_pump);

    false
}

fn game_logic<TInput: IInput>(input: &TInput) -> bool {
    // thread::sleep(Duration::from_millis(1000));
    // ris_log::debug!("{}", frame_buffer.fps());

    let buttons = input.general().buttons();
    if buttons.down() != 0 || buttons.up() != 0 {
        ris_log::debug!("{:#034b}", buttons.hold());
    }

    false
}

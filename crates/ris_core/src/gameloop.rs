use std::{
    thread,
    time::{Duration, Instant},
};

use ris_data::info::ipackage_info::IPackageInfo;
use ris_input::{buttons::IButtons, input::IInput};

use sdl2::event::Event;
use sdl2::EventPump;

use crate::bootstrapper::GlobalContainer;

pub enum GameloopState {
    Running,
    WantsToQuit,
    Error(String),
}

pub fn run_one_frame<TPackageInfo: IPackageInfo>(
    container: &mut GlobalContainer<TPackageInfo>,
) -> GameloopState {
    let now = Instant::now();

    let pump_wants_to_quit = pump_events(&mut container.input, &mut container.event_pump);
    let game_wants_to_quit = game_logic(&container.input);

    let delta = now.elapsed();

    container.frame_buffer.add(delta);

    if pump_wants_to_quit || game_wants_to_quit {
        GameloopState::WantsToQuit
    } else {
        GameloopState::Running
    }
}

fn pump_events<TInput: IInput>(input: &mut TInput, event_pump: &mut EventPump) -> bool {
    input.pre_update();

    for event in event_pump.poll_iter() {
        // ris_log::debug!("{:?}", event);

        if let Event::Quit { .. } = event {
            return true;
        };

        input.update(&event);
    }

    input.post_update(event_pump);

    false
}

fn game_logic<TInput: IInput>(input: &TInput) -> bool {
    thread::sleep(Duration::from_millis(50));
    // ris_log::debug!("{}", frame_buffer.fps());

    ris_log::debug!("{:#034b}", input.general().buttons().hold());

    false
}

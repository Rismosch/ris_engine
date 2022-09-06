use std::cell::{Ref, RefMut};

use ris_data::gameloop::{
    frame_data::FrameData, gameloop_state::GameloopState, input_data::InputData,
};
use sdl2::{EventPump, event::Event};

pub fn run(current: &mut InputData, previous: &InputData, frame: &FrameData, event_pump: &mut EventPump) -> GameloopState {
    
    
    ris_log::debug!("{} {}", frame.number(), frame.fps());

    for event in event_pump.poll_iter() {
        // ris_log::trace!("{:?}", event);

        if let Event::Quit { .. } = event {
            return GameloopState::WantsToQuit;
        };
    }

    GameloopState::WantsToContinue
}

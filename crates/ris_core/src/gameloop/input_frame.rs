use ris_data::gameloop::{
    frame_data::FrameData, gameloop_state::GameloopState, input_data::InputData,
};
use sdl2::{event::Event, EventPump};

pub fn run(
    _current: &mut InputData,
    _previous: &InputData,
    _frame: &FrameData,
    event_pump: &mut EventPump,
) -> GameloopState {

    
    for event in event_pump.poll_iter() {
        ris_log::trace!("{:?}", event);

        if let Event::Quit { .. } = event {
            return GameloopState::WantsToQuit;
        };
    }

    GameloopState::WantsToContinue
}

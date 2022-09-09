use ris_data::gameloop::{
    frame_data::FrameData, gameloop_state::GameloopState, input_data::InputData,
};
use ris_input::mouse_logic::{pre_update_mouse, update_mouse, post_update_mouse};
use sdl2::{event::Event, EventPump};

pub fn run(
    current: &mut InputData,
    previous: &InputData,
    _frame: &FrameData,
    event_pump: &mut EventPump,
) -> GameloopState {
    pre_update_mouse(&mut current.mouse);
    
    for event in event_pump.poll_iter() {
        // ris_log::trace!("{:?}", event);

        if let Event::Quit { .. } = event {
            return GameloopState::WantsToQuit;
        };

        update_mouse(&mut current.mouse, &event);
    }

    post_update_mouse(&mut current.mouse, &previous.mouse, event_pump.mouse_state());

    GameloopState::WantsToContinue
}

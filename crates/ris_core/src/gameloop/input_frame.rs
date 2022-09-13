use ris_data::gameloop::{
    frame_data::FrameData, gameloop_state::GameloopState, input_data::InputData,
};
use ris_input::mouse_logic::{post_update_mouse, pre_update_mouse, update_mouse};
use sdl2::{event::Event, EventPump};

pub fn run(
    mut current: InputData,
    previous: &InputData,
    _frame: &FrameData,
    event_pump: &mut EventPump,
) -> (InputData, GameloopState) {
    pre_update_mouse(&mut current.mouse);

    for event in event_pump.poll_iter() {
        ris_log::trace!("fps: {} event: {:?}", _frame.fps(), event);

        if let Event::Quit { .. } = event {
            return (current, GameloopState::WantsToQuit);
        };

        update_mouse(&mut current.mouse, &event);
    }

    post_update_mouse(
        &mut current.mouse,
        &previous.mouse,
        event_pump.mouse_state(),
    );

    (current, GameloopState::WantsToContinue)
}

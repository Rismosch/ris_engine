use ris_data::gameloop::{
    frame_data::FrameData, gameloop_state::GameloopState, input_data::InputData,
    logic_data::LogicData,
};
use ris_jobs::job_system;

pub fn run(
    current: LogicData,
    _previous: &LogicData,
    _input: &InputData,
    _frame: &FrameData,
) -> (LogicData, GameloopState) {
    if _input.general.buttons.down() != 0 {
        ris_log::debug!(
            "{:#034b} {} {}",
            _input.general.buttons.down(),
            job_system::thread_index(),
            _frame.fps()
        );
    }

    (current, GameloopState::WantsToContinue)
}

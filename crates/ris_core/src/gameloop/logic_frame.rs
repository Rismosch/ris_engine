// use std::{thread, time::Duration};

use ris_data::gameloop::{
    frame_data::FrameData, gameloop_state::GameloopState, input_data::InputData,
    logic_data::LogicData,
};
// use ris_jobs::job_system;

pub fn run(
    current: LogicData,
    _previous: &LogicData,
    _input: &InputData,
    _frame: &'static FrameData,
) -> (LogicData, GameloopState) {
    // thread::sleep(Duration::from_millis(50));

    // ris_log::debug!("{:#034b} {} {}", _input.mouse.xrel, job_system::thread_index(), _frame.fps());

    (current, GameloopState::WantsToContinue)
}

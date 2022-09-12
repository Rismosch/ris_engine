use std::{thread, time::Duration};

use ris_data::gameloop::{
    frame_data::FrameData, gameloop_state::GameloopState, input_data::InputData,
    logic_data::LogicData,
};
use ris_jobs::job_system;

pub fn run(
    current: LogicData,
    _previous: &LogicData,
    input: &InputData,
    frame: &FrameData,
) -> (LogicData, GameloopState) {
    // thread::sleep(Duration::from_millis(50));

    // ris_log::debug!("{:#034b} {} {}", input.mouse.buttons.hold(), job_system::thread_index(), frame.fps());

    (current, GameloopState::WantsToContinue)
}

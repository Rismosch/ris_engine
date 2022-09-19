use ris_data::gameloop::{
    frame_data::FrameData, gameloop_state::GameloopState, logic_data::LogicData,
    output_data::OutputData,
};
use ris_jobs::job_cell::Ref;

pub fn run(
    current: OutputData,
    _previous: Ref<OutputData>,
    _logic: Ref<LogicData>,
    _frame: Ref<FrameData>,
) -> (OutputData, GameloopState) {
    (current, GameloopState::WantsToContinue)
}

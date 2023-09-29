use ris_data::gameloop::frame_data::FrameData;
use ris_data::gameloop::gameloop_state::GameloopState;
use ris_data::gameloop::input_data::InputData;
use ris_data::gameloop::logic_data::LogicData;
use ris_data::gameloop::output_data::OutputData;
use ris_video::video::Video;

pub struct OutputFrame {
    video: Video,
}

impl OutputFrame {
    pub fn new(video: Video) -> Self {
        Self { video }
    }

    pub fn run(
        &mut self,
        _current: &mut OutputData,
        _previous: &OutputData,
        input: &InputData,
        logic: &LogicData,
        _frame: &FrameData,
    ) -> GameloopState {
        if input.window_size_changed.is_some() || logic.new_viewport_requested {
            self.video.recreate_viewport();
        }

        match self.video.update(&logic.scene) {
            Ok(()) => GameloopState::WantsToContinue,
            Err(e) => GameloopState::Error(e),
        }
    }
}

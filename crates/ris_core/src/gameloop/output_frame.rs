use ris_data::gameloop::{
    frame_data::FrameData, gameloop_state::GameloopState, input_data::InputData,
    logic_data::LogicData, output_data::OutputData,
};
use ris_video::video::{DrawState, Video};

pub struct OutputFrame {
    video: Video,
    recreate_swapchain: bool,
}

impl OutputFrame {
    pub fn new(video: Video) -> Result<Self, String> {
        Ok(Self {
            video,
            recreate_swapchain: false,
        })
    }

    pub fn run(
        &mut self,
        _current: &mut OutputData,
        _previous: &OutputData,
        input: &InputData,
        _logic: &LogicData,
        _frame: &FrameData,
    ) -> GameloopState {
        if self.video.can_draw() {
            let window_size_changed = input.window_size_changed.is_some();
            let recreate_swapchain = self.recreate_swapchain;
            if window_size_changed || recreate_swapchain {
                ris_log::trace!("recreate swapchain...");

                if let Err(error) = self.video.recreate_swapchain(window_size_changed) {
                    return GameloopState::Error(error);
                }

                self.recreate_swapchain = false;

                ris_log::debug!("swapchain recreated");
            }

            match self.video.draw() {
                DrawState::Ok => (),
                DrawState::WantsToRecreateSwapchain => self.recreate_swapchain = true,
                DrawState::Err(e) => return GameloopState::Error(e),
            }
        }

        GameloopState::WantsToContinue
    }
}

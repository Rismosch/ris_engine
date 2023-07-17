use ris_data::gameloop::frame_data::FrameData;
use ris_data::gameloop::gameloop_state::GameloopState;
use ris_data::gameloop::input_data::InputData;
use ris_data::gameloop::logic_data::LogicData;
use ris_data::gameloop::output_data::OutputData;
use ris_math::matrix4x4::Matrix4x4;
use ris_video::video::DrawState;
use ris_video::video::Video;

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
        logic: &LogicData,
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

            // 1. local to world space => model matrix
            // 2. world to view space => view matrix
            // 3. view to clip space => projection matrix
            // 4. clip to screen space => viewport transform

            let camera_transformation =
                Matrix4x4::transformation(logic.camera_rotation, logic.camera_position);

            match self.video.draw() {
                DrawState::Ok => (),
                DrawState::WantsToRecreateSwapchain => self.recreate_swapchain = true,
                DrawState::Err(e) => return GameloopState::Error(e),
            }
        }

        GameloopState::WantsToContinue
    }
}

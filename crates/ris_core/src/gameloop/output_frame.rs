use ris_data::gameloop::frame_data::FrameData;
use ris_data::gameloop::gameloop_state::GameloopState;
use ris_data::gameloop::input_data::InputData;
use ris_data::gameloop::logic_data::LogicData;
use ris_data::gameloop::output_data::OutputData;
use ris_debug::imgui::Imgui;
use ris_video::video::Video;

pub struct OutputFrame {
    video: Video,
}

impl OutputFrame {
    pub fn new(video: Video) -> Result<Self, String> {
        Ok(Self { video })
    }

    pub fn run(
        &mut self,
        _current: &mut OutputData,
        _previous: &OutputData,
        input: &InputData,
        logic: &LogicData,
        _frame: &FrameData,
    ) -> GameloopState {
        if input.window_size_changed.is_some() {
            self.video.on_window_resize();
        }

        match self.video.update(&logic.scene) {
            Ok(()) => GameloopState::WantsToContinue,
            Err(e) => GameloopState::Error(e),
        }
    }

    pub fn render_imgui(&mut self, imgui: &mut Imgui, input: &InputData){
        let mut ui = imgui.prepare_and_create_new_frame(
            self.video.window(),
            &input.mouse,
        );

        ui.text("Hello world!");
        ui.text("こんにちは世界！");
        ui.text("This...is...imgui-rs!");
        ui.separator();
        let mouse_pos = ui.io().mouse_pos;
        ui.text(format!(
            "Mouse Position: ({:.1},{:.1})",
            mouse_pos[0], mouse_pos[1]
        ));

        imgui.render(&self.video);
    }
}

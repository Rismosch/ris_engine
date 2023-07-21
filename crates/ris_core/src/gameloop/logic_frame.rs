use ris_data::gameloop::frame_data::FrameData;
use ris_data::gameloop::gameloop_state::GameloopState;
use ris_data::gameloop::input_data::InputData;
use ris_data::gameloop::logic_data::LogicData;
use ris_data::input::action;

#[derive(Default)]
pub struct LogicFrame {}

impl LogicFrame {
    pub fn run(
        &mut self,
        current: &mut LogicData,
        previous: &LogicData,
        input: &InputData,
        frame: &FrameData,
    ) -> GameloopState {
        //let camera_speed = 1.;
        //
        //if input.general.buttons.is_hold(action::CAMERA_UP) {
        //    self.camera_vertical_angle += camera_speed * frame.delta();
        //}
        //
        //if input.general.buttons.is_hold(action::CAMERA_DOWN) {
        //    self.camera_vertical_angle -= camera_speed * frame.delta();
        //}
        //
        //if input.general.buttons.is_hold(action::CAMERA_LEFT) {
        //    self.camera_horizontal_angle += camera_speed * frame.delta();
        //}
        //
        //if input.general.buttons.is_hold(action::CAMERA_RIGHT) {
        //    self.camera_horizontal_angle -= camera_speed * frame.delta();
        //}
        //
        //while self.camera_horizontal_angle < 0. {
        //    self.camera_horizontal_angle += ris_math::PI_2;
        //}
        //while self.camera_horizontal_angle > ris_math::PI_2 {
        //    self.camera_horizontal_angle -= ris_math::PI_2;
        //}
        //self.camera_vertical_angle = ris_math::clamp(
        //    self.camera_vertical_angle,
        //    -ris_math::PI_0_5,
        //    ris_math::PI_0_5,
        //);
        //
        //let rotation1 = Quaternion::from_angle_axis(self.camera_horizontal_angle, vector3::UP);
        //let rotation2 = Quaternion::from_angle_axis(self.camera_vertical_angle, vector3::RIGHT);
        //current.camera_rotation = rotation1 * rotation2;

        current.scene = previous.scene.clone();
        let scene = &mut current.scene;

        if input.general.buttons.is_down(action::CAMERA_UP) {
            scene.debug_y += 1;
        }

        if input.general.buttons.is_down(action::CAMERA_DOWN) {
            scene.debug_y -= 1;
        }

        if input.general.buttons.is_down(action::CAMERA_LEFT) {
            scene.debug_x -= 1;
        }

        if input.general.buttons.is_down(action::CAMERA_RIGHT) {
            scene.debug_x += 1;
        }

        if scene.debug_x < 0 {
            scene.debug_x = 0
        }
        if scene.debug_x > 3 {
            scene.debug_x = 3
        }
        if scene.debug_y < 0 {
            scene.debug_y = 0
        }
        if scene.debug_y > 3 {
            scene.debug_y = 3
        }

        if input.general.buttons.is_down(action::ANY) {
            ris_log::debug!(
                "{} {} | {}s {}fps",
                scene.debug_x,
                scene.debug_y,
                frame.delta(),
                frame.fps()
            );
        }

        GameloopState::WantsToContinue
    }
}

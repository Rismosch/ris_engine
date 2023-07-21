use ris_data::gameloop::frame_data::FrameData;
use ris_data::gameloop::gameloop_state::GameloopState;
use ris_data::gameloop::input_data::InputData;
use ris_data::gameloop::logic_data::LogicData;
use ris_data::input::action;
use ris_math::quaternion::Quaternion;
use ris_math::vector3;

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
        current.camera_horizontal_angle = previous.camera_horizontal_angle;
        current.camera_vertical_angle = previous.camera_vertical_angle;
        current.scene = previous.scene.clone();
        let scene = &mut current.scene;

        let camera_speed = 2.;

        if input.general.buttons.is_down(action::OK) {
            current.camera_horizontal_angle = 0.0;
            current.camera_vertical_angle = 0.0;
        }

        if input.general.buttons.is_hold(action::CAMERA_UP) {
            current.camera_vertical_angle += camera_speed * frame.delta();
        }

        if input.general.buttons.is_hold(action::CAMERA_DOWN) {
            current.camera_vertical_angle -= camera_speed * frame.delta();
        }

        if input.general.buttons.is_hold(action::CAMERA_LEFT) {
            current.camera_horizontal_angle += camera_speed * frame.delta();
        }

        if input.general.buttons.is_hold(action::CAMERA_RIGHT) {
            current.camera_horizontal_angle -= camera_speed * frame.delta();
        }

        while current.camera_horizontal_angle < 0. {
            current.camera_horizontal_angle += ris_math::PI_2;
        }
        while current.camera_horizontal_angle > ris_math::PI_2 {
            current.camera_horizontal_angle -= ris_math::PI_2;
        }
        current.camera_vertical_angle = ris_math::clamp(
            current.camera_vertical_angle,
            -ris_math::PI_0_5,
            ris_math::PI_0_5,
        );

        let rotation1 = Quaternion::from_angle_axis(current.camera_horizontal_angle, vector3::UP);
        let rotation2 = Quaternion::from_angle_axis(current.camera_vertical_angle, vector3::RIGHT);
        scene.camera_rotation = rotation1 * rotation2;

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

        if input.general.buttons.is_hold(action::ANY) {
            ris_log::debug!(
                "horizontal: {} vertical: {} | {}s {}fps",
                current.camera_horizontal_angle,
                current.camera_vertical_angle,
                frame.delta(),
                frame.fps()
            );
        }

        GameloopState::WantsToContinue
    }
}

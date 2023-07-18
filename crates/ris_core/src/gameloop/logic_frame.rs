use ris_data::gameloop::frame_data::FrameData;
use ris_data::gameloop::gameloop_state::GameloopState;
use ris_data::gameloop::input_data::InputData;
use ris_data::gameloop::logic_data::LogicData;
use ris_data::input::action;
use ris_math::quaternion::Quaternion;
use ris_math::vector3;

#[derive(Default)]
pub struct LogicFrame {
    camera_horizontal_angle: f32,
    camera_vertical_angle: f32,

    debug_x: i32,
    debug_y: i32,
}

impl LogicFrame {
    pub fn run(
        &mut self,
        current: &mut LogicData,
        _previous: &LogicData,
        input: &InputData,
        frame: &FrameData,
    ) -> GameloopState {
        let camera_speed = 1.;

        if input.general.buttons.is_hold(action::CAMERA_UP) {
            self.camera_vertical_angle += camera_speed * frame.delta();
        }

        if input.general.buttons.is_hold(action::CAMERA_DOWN) {
            self.camera_vertical_angle -= camera_speed * frame.delta();
        }

        if input.general.buttons.is_hold(action::CAMERA_LEFT) {
            self.camera_horizontal_angle += camera_speed * frame.delta();
        }

        if input.general.buttons.is_hold(action::CAMERA_RIGHT) {
            self.camera_horizontal_angle -= camera_speed * frame.delta();
        }

        while self.camera_horizontal_angle < 0. {
            self.camera_horizontal_angle += ris_math::PI_2;
        }
        while self.camera_horizontal_angle > ris_math::PI_2 {
            self.camera_horizontal_angle -= ris_math::PI_2;
        }
        self.camera_vertical_angle = ris_math::clamp(
            self.camera_vertical_angle,
            -ris_math::PI_0_5,
            ris_math::PI_0_5,
        );

        let rotation1 = Quaternion::from_angle_axis(self.camera_horizontal_angle, vector3::UP);
        let rotation2 = Quaternion::from_angle_axis(self.camera_vertical_angle, vector3::RIGHT);
        current.camera_rotation = rotation1 * rotation2;

        //let forward = current.camera_rotation.rotate(vector3::FORWARD);
        //ris_log::debug!("{:?}", forward);
        
        if input.general.buttons.is_down(action::CAMERA_UP) {
            self.debug_y += 1;
        }

        if input.general.buttons.is_down(action::CAMERA_DOWN) {
            self.debug_y -= 1;
        }
        
        if input.general.buttons.is_down(action::CAMERA_LEFT) {
            self.debug_x -= 1;
        }

        if input.general.buttons.is_down(action::CAMERA_RIGHT) {
            self.debug_x += 1;
        }

        if self.debug_x < 0 {
            self.debug_x = 0
        }
        if self.debug_x > 3 {
            self.debug_x = 3
        }
        if self.debug_y < 0 {
            self.debug_y = 0
        }
        if self.debug_y > 3 {
            self.debug_y = 3
        }

        if input.general.buttons.is_down(action::ANY) {
            ris_log::debug!("{} {}", self.debug_x, self.debug_y);
        }

        GameloopState::WantsToContinue
    }
}

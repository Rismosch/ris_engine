use ris_data::gameloop::frame_data::FrameData;
use ris_data::gameloop::gameloop_state::GameloopState;
use ris_data::gameloop::input_data::InputData;
use ris_data::gameloop::logic_data::LogicData;
use ris_data::input::action;
use ris_math::matrix4x4::Matrix4x4;
use ris_math::quaternion::Quaternion;
use ris_math::vector3;
use ris_math::vector3::Vector3;

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

        let rotation_speed = 2. * frame.delta();
        let movement_speed = 2. * frame.delta();
        let mouse_speed = 20. * frame.delta();

        if input.mouse.buttons.is_hold(action::OK) {
            current.camera_vertical_angle += mouse_speed * input.mouse.yrel as f32;
            current.camera_horizontal_angle += mouse_speed * input.mouse.xrel as f32;
        } else if input.general.buttons.is_down(action::OK) {
            current.camera_horizontal_angle = 0.0;
            current.camera_vertical_angle = 0.0;
            scene.camera_position = Vector3::new(0., -2., 0.);
        }

        if input.general.buttons.is_hold(action::CAMERA_UP) {
            current.camera_vertical_angle += rotation_speed;
        }

        if input.general.buttons.is_hold(action::CAMERA_DOWN) {
            current.camera_vertical_angle -= rotation_speed;
        }

        if input.general.buttons.is_hold(action::CAMERA_LEFT) {
            current.camera_horizontal_angle += rotation_speed;
        }

        if input.general.buttons.is_hold(action::CAMERA_RIGHT) {
            current.camera_horizontal_angle -= rotation_speed;
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

        if input.general.buttons.is_hold(action::MOVE_UP) {
            let forward = scene.camera_rotation.rotate(vector3::FORWARD);
            scene.camera_position += movement_speed * forward;
        }

        if input.general.buttons.is_hold(action::MOVE_DOWN) {
            let forward = scene.camera_rotation.rotate(vector3::FORWARD);
            scene.camera_position -= movement_speed * forward;
        }

        if input.general.buttons.is_hold(action::MOVE_LEFT) {
            let right = scene.camera_rotation.rotate(vector3::RIGHT);
            scene.camera_position += movement_speed * right;
        }

        if input.general.buttons.is_hold(action::MOVE_RIGHT) {
            let right = scene.camera_rotation.rotate(vector3::RIGHT);
            scene.camera_position -= movement_speed * right;
        }

        if input.general.buttons.is_hold(action::ANY) {
            //let quaternion = scene.camera_rotation;
            //let matrix = Matrix4x4::transformation(quaternion, vector3::ZERO);
            //let my_vector = Vector3::new(0.0,1.0,0.0);
            //let rotated1 = quaternion.rotate(my_vector);
            //let rotated2 = matrix.rotate(my_vector);

            //ris_log::debug!(
            //    "horizontal: {} vertical: {} | rotated1: {:?} rotated2: {:?} position: {:?} | {}s {}fps",
            //    current.camera_horizontal_angle,
            //    current.camera_vertical_angle,
            //    rotated1,
            //    rotated2,
            //    scene.camera_position,
            //    frame.delta(),
            //    frame.fps()
            //);
            
            let rotation = scene.camera_rotation;
            let position = scene.camera_position;

            let matrix = Matrix4x4::view(rotation, position);
            let position_zero = matrix.rotate_and_transform(position);

            ris_log::debug!("{:?} {:?}", position, rotation);

        }

        GameloopState::WantsToContinue
    }
}

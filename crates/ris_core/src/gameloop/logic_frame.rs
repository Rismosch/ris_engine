use ris_asset::asset_loader::AssetLoader;
use ris_asset::asset_loader::AssetId;
use ris_data::gameloop::frame_data::FrameData;
use ris_data::gameloop::gameloop_state::GameloopState;
use ris_data::gameloop::input_data::InputData;
use ris_data::gameloop::logic_data::LogicData;
use ris_data::input::action;
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
        asset_loader: &AssetLoader,
    ) -> GameloopState {
        current.camera_horizontal_angle = previous.camera_horizontal_angle;
        current.camera_vertical_angle = previous.camera_vertical_angle;
        current.scene = previous.scene.clone();
        let scene = &mut current.scene;

        let rotation_speed = 2. * frame.delta();
        let movement_speed = 2. * frame.delta();
        let mouse_speed = 20. * frame.delta();

        if input.mouse.buttons.is_hold(action::OK) {
            current.camera_vertical_angle -= mouse_speed * input.mouse.yrel as f32;
            current.camera_horizontal_angle -= mouse_speed * input.mouse.xrel as f32;
        } else if input.general.buttons.is_down(action::OK) {
            current.camera_horizontal_angle = 0.0;
            current.camera_vertical_angle = 0.0;
            scene.camera_position = Vector3::new(0., -1., 0.);

            let id = AssetId::Directory(String::from("hello world"));
            let future = asset_loader.load(id);
            let result = future.wait();
            match result {
                Err(error) => ris_log::error!("failed to load asset: {}", error),
                Ok(bytes) => {
                    let string_result = String::from_utf8(bytes);
                    match string_result {
                        Err(error) => ris_log::error!("asset is not a valid utf8 string"),
                        Ok(string) => ris_log::info!("asset loaded: {}", string),
                    }
                },
            }
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

        if input.general.buttons.is_down(action::DEBUG_UP) {
            scene.debug_y -= 1;
        }

        if input.general.buttons.is_down(action::DEBUG_DOWN) {
            scene.debug_y += 1;
        }

        if input.general.buttons.is_down(action::DEBUG_LEFT) {
            scene.debug_x -= 1;
        }

        if input.general.buttons.is_down(action::DEBUG_RIGHT) {
            scene.debug_x += 1;
        }

        if scene.debug_x < 0 {
            scene.debug_x = 0;
        }
        if scene.debug_x > 3 {
            scene.debug_x = 3;
        }

        if scene.debug_y < 0 {
            scene.debug_y = 0;
        }
        if scene.debug_y > 3 {
            scene.debug_y = 3;
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

        let rotation1 = Quaternion::from_angle_axis(current.camera_vertical_angle, vector3::RIGHT);
        let rotation2 = Quaternion::from_angle_axis(current.camera_horizontal_angle, vector3::UP);
        scene.camera_rotation = rotation2 * rotation1;

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
            scene.camera_position -= movement_speed * right;
        }

        if input.general.buttons.is_hold(action::MOVE_RIGHT) {
            let right = scene.camera_rotation.rotate(vector3::RIGHT);
            scene.camera_position += movement_speed * right;
        }

        //if input.general.buttons.is_hold(action::ANY) {
        //    let rotation_matrix = ris_math::matrix4x4::Matrix4x4::rotation(scene.camera_rotation);
        //    let camera_forward_matrix = rotation_matrix.rotate(vector3::FORWARD);
        //    let camera_forward_quaternion = scene.camera_rotation.rotate(vector3::FORWARD);
        //    let camera_forward_view =
        //        ris_math::matrix4x4::Matrix4x4::view(scene.camera_position, scene.camera_rotation)
        //            .rotate(vector3::FORWARD);
        //    ris_log::debug!(
        //        "{:?} {:?} {:?} {:?} | {} {} | {}s {}fps",
        //        scene.camera_position,
        //        camera_forward_matrix,
        //        camera_forward_quaternion,
        //        camera_forward_view,
        //        current.camera_horizontal_angle,
        //        current.camera_vertical_angle,
        //        frame.delta(),
        //        frame.fps()
        //    );
        //}

        GameloopState::WantsToContinue
    }
}

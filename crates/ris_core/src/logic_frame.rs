use std::f32::consts::PI;

use sdl2::keyboard::Scancode;

use ris_data::gameloop::frame::Frame;
use ris_data::gameloop::gameloop_state::GameloopState;
use ris_data::god_state::GodState;
use ris_data::input::action;
use ris_error::RisResult;
use ris_math::quaternion::Quat;
use ris_math::vector::Vec3;

#[derive(Default)]
pub struct LogicFrame {
    // camera
    camera_horizontal_angle: f32,
    camera_vertical_angle: f32,
}

impl LogicFrame {
    pub fn run(&mut self, frame: Frame, state: &mut GodState) -> RisResult<GameloopState> {
        let input = &state.input;

        if state.debug_ui_is_focused {
            return Ok(GameloopState::WantsToContinue);
        }

        let rotation_speed = 2. * frame.average_seconds();
        let movement_speed = 2. * frame.average_seconds();

        if input.mouse.buttons.is_hold(action::OK) {
            let yrel = 0.01 * input.mouse.yrel as f32;
            let xrel = 0.01 * input.mouse.xrel as f32;
            self.camera_vertical_angle -= yrel;
            self.camera_horizontal_angle -= xrel;
        } else if input.general.buttons.is_down(action::OK) {
            self.camera_horizontal_angle = 0.0;
            self.camera_vertical_angle = 0.0;
            state.camera.position = Vec3::backward();
        }

        if input.general.buttons.is_hold(action::CAMERA_UP) {
            self.camera_vertical_angle += rotation_speed;
        }

        if input.general.buttons.is_hold(action::CAMERA_DOWN) {
            self.camera_vertical_angle -= rotation_speed;
        }

        if input.general.buttons.is_hold(action::CAMERA_LEFT) {
            self.camera_horizontal_angle += rotation_speed;
        }

        if input.general.buttons.is_hold(action::CAMERA_RIGHT) {
            self.camera_horizontal_angle -= rotation_speed;
        }

        while self.camera_horizontal_angle < 0. {
            self.camera_horizontal_angle += 2. * PI;
        }
        while self.camera_horizontal_angle > 2. * PI {
            self.camera_horizontal_angle -= 2. * PI;
        }
        self.camera_vertical_angle = f32::clamp(self.camera_vertical_angle, -0.5 * PI, 0.5 * PI);

        let rotation1 = Quat::from((self.camera_vertical_angle, Vec3::right()));
        let rotation2 = Quat::from((self.camera_horizontal_angle, Vec3::up()));
        state.camera.rotation = rotation2 * rotation1;

        if input.general.buttons.is_hold(action::MOVE_UP) {
            let forward = state.camera.rotation.rotate(Vec3::forward());
            state.camera.position += movement_speed * forward;
        }

        if input.general.buttons.is_hold(action::MOVE_DOWN) {
            let forward = state.camera.rotation.rotate(Vec3::forward());
            state.camera.position -= movement_speed * forward;
        }

        if input.general.buttons.is_hold(action::MOVE_LEFT) {
            let right = state.camera.rotation.rotate(Vec3::right());
            state.camera.position -= movement_speed * right;
        }

        if input.general.buttons.is_hold(action::MOVE_RIGHT) {
            let right = state.camera.rotation.rotate(Vec3::right());
            state.camera.position += movement_speed * right;
        }

        if input.keyboard.keys.is_down(Scancode::F) {
            println!(
                "{:?} ({} fps)",
                frame.average_duration(),
                frame.average_fps()
            );
        }

        if frame.number() % 100 == 0 {
            println!(
                "{:?} ({} fps)",
                frame.average_duration(),
                frame.average_fps()
            );
        }

        Ok(GameloopState::WantsToContinue)
    }
}

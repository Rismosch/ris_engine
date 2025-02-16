use std::f32::consts::PI;

use ris_data::ecs::script_prelude::*;
use ris_data::input::action;
use ris_math::quaternion::Quat;
use ris_math::vector::Vec3;

#[derive(Debug, Default)]
pub struct FlyCameraScript {
    yaw: f32,
    pitch: f32,
}

impl Script for FlyCameraScript {
    fn start(&mut self, _data: ScriptStartEndData) -> RisResult<()> {
        Ok(())
    }

    fn update(&mut self, data: ScriptUpdateData) -> RisResult<()> {
        let ScriptUpdateData {
            game_object: _,
            frame,
            state,
        } = data;

        let input = &state.input;

        if state.debug_ui_is_focused {
            return Ok(());
        }

        let rotation_speed = 2. * frame.average_seconds();
        let movement_speed = 2. * frame.average_seconds();

        if input.mouse.buttons.is_hold(action::OK) {
            let xrel = 0.01 * input.mouse.xrel as f32;
            let yrel = 0.01 * input.mouse.yrel as f32;
            self.yaw -= xrel;
            self.pitch -= yrel;
        } else if input.general.buttons.is_down(action::OK) {
            self.yaw = 0.0;
            self.pitch = 0.0;
            state.camera.borrow_mut().position = Vec3::backward();
        }

        if input.general.buttons.is_hold(action::CAMERA_UP) {
            self.pitch += rotation_speed;
        }

        if input.general.buttons.is_hold(action::CAMERA_DOWN) {
            self.pitch -= rotation_speed;
        }

        if input.general.buttons.is_hold(action::CAMERA_LEFT) {
            self.yaw += rotation_speed;
        }

        if input.general.buttons.is_hold(action::CAMERA_RIGHT) {
            self.yaw -= rotation_speed;
        }

        while self.yaw < 0. {
            self.yaw += 2. * PI;
        }
        while self.yaw > 2. * PI {
            self.yaw -= 2. * PI;
        }
        self.pitch = f32::clamp(self.pitch, -0.5 * PI, 0.5 * PI);

        let rotation1 = Quat::from((self.pitch, Vec3::right()));
        let rotation2 = Quat::from((self.yaw, Vec3::up()));
        state.camera.borrow_mut().rotation = rotation2 * rotation1;

        if input.general.buttons.is_hold(action::MOVE_UP) {
            let forward = state.camera.borrow().rotation.rotate(Vec3::forward());
            state.camera.borrow_mut().position += movement_speed * forward;
        }

        if input.general.buttons.is_hold(action::MOVE_DOWN) {
            let forward = state.camera.borrow().rotation.rotate(Vec3::forward());
            state.camera.borrow_mut().position -= movement_speed * forward;
        }

        if input.general.buttons.is_hold(action::MOVE_LEFT) {
            let right = state.camera.borrow().rotation.rotate(Vec3::right());
            state.camera.borrow_mut().position -= movement_speed * right;
        }

        if input.general.buttons.is_hold(action::MOVE_RIGHT) {
            let right = state.camera.borrow().rotation.rotate(Vec3::right());
            state.camera.borrow_mut().position += movement_speed * right;
        }

        if frame.number() % 100 == 0 {
            println!(
                "{:?} ({} fps)",
                frame.average_duration(),
                frame.average_fps()
            );
        }

        Ok(())
    }

    fn end(&mut self, _data: ScriptStartEndData) -> RisResult<()> {
        Ok(())
    }

    fn inspect(&mut self, _data: ScriptInspectData) -> RisResult<()> {
        Ok(())
    }

    fn serialize(&mut self, _stream: &mut SceneWriter) -> RisResult<()> {
        Ok(())
    }

    fn deserialize(&mut self, _stream: &mut SceneReader) -> RisResult<()> {
        Ok(())
    }
}

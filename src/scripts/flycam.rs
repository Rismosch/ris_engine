use std::f32::consts::PI;

use sdl2::keyboard::Scancode;

use ris_data::ecs::script_prelude::*;
use ris_data::input::action;
use ris_math::quaternion::Quat;
use ris_math::vector::Vec3;

#[derive(Debug)]
pub struct FlyCam {
    yaw: f32,
    pitch: f32,
    translation_speed_in_meters_per_second: f32,
}

impl Default for FlyCam {
    fn default() -> Self {
        Self {
            yaw: 0.0,
            pitch: 0.0,
            translation_speed_in_meters_per_second: 10.0,
        }
    }
}

impl Script for FlyCam {
    fn start(&mut self, _data: ScriptStartEndData) -> RisResult<()> {
        Ok(())
    }

    fn update(&mut self, data: ScriptUpdateData) -> RisResult<()> {
        let ScriptUpdateData {
            game_object: _,
            frame,
            state,
        } = data;

        if state.debug_ui_is_focused {
            return Ok(());
        }

        let rotation_speed = 2. * frame.average_seconds();
        let translation_speed =
            frame.average_seconds() * self.translation_speed_in_meters_per_second;

        let mut camera = state.camera.borrow_mut();

        if state.input.mouse.buttons.is_hold(action::OK) {
            let yrel = 0.01 * state.input.mouse.yrel as f32;
            let xrel = 0.01 * state.input.mouse.xrel as f32;
            self.yaw -= xrel;
            self.pitch -= yrel;
        } else if state.input.general.buttons.is_down(action::OK) {
            self.yaw = 0.0;
            self.pitch = 0.0;
            camera.position = Vec3::backward();
        }

        if state.input.general.buttons.is_hold(action::CAMERA_UP) {
            self.pitch += rotation_speed;
        }

        if state.input.general.buttons.is_hold(action::CAMERA_DOWN) {
            self.pitch -= rotation_speed;
        }

        if state.input.general.buttons.is_hold(action::CAMERA_LEFT) {
            self.yaw += rotation_speed;
        }

        if state.input.general.buttons.is_hold(action::CAMERA_RIGHT) {
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
        camera.rotation = rotation2 * rotation1;

        let mut translation_direction = Vec3::init(0.0);
        if state.input.general.buttons.is_hold(action::MOVE_UP) {
            let forward = camera.rotation.rotate(Vec3::forward());
            translation_direction += forward;
        }

        if state.input.general.buttons.is_hold(action::MOVE_DOWN) {
            let forward = camera.rotation.rotate(Vec3::forward());
            translation_direction -= forward;
        }

        if state.input.general.buttons.is_hold(action::MOVE_LEFT) {
            let right = camera.rotation.rotate(Vec3::right());
            translation_direction -= right;
        }

        if state.input.general.buttons.is_hold(action::MOVE_RIGHT) {
            let right = camera.rotation.rotate(Vec3::right());
            translation_direction += right;
        }

        if translation_direction.length() > 0.5 {
            let translation = translation_direction.normalize() * translation_speed;
            camera.position += translation;
        }

        if state.input.keyboard.keys.is_down(Scancode::F) {
            eprintln!(
                "average {:?} ({} fps)",
                frame.average_duration(),
                frame.average_fps(),
            );
        }

        Ok(())
    }

    fn end(&mut self, _data: ScriptStartEndData) -> RisResult<()> {
        Ok(())
    }

    fn inspect(&mut self, data: ScriptInspectData) -> RisResult<()> {
        let ScriptInspectData { id, state, .. } = data;

        let mut camera = state.camera.borrow_mut();

        ris_core::inspector_util::drag_vec3(
            format!("camera position##{}", id),
            &mut camera.position,
        )?;
        ris_core::inspector_util::drag(format!("far##{}", id), &mut camera.far)?;
        ris_core::inspector_util::drag(format!("near##{}", id), &mut camera.near)?;

        ris_core::inspector_util::drag(
            format!("translation speed (m/s)##{}", id),
            &mut self.translation_speed_in_meters_per_second,
        )?;

        Ok(())
    }

    fn serialize(&mut self, _stream: &mut SceneWriter) -> RisResult<()> {
        Ok(())
    }

    fn deserialize(&mut self, _stream: &mut SceneReader) -> RisResult<()> {
        Ok(())
    }
}

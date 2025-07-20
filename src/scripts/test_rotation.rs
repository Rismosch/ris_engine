use std::f32::consts::PI;

use ris_data::ecs::script_prelude::*;
use ris_math::quaternion::Quat;
use ris_math::vector::Vec3;

#[derive(Debug)]
pub struct TestRotationScript {
    pub rotation_axis: Vec3,
    pub rotation_speed: f32,
}

impl Default for TestRotationScript {
    fn default() -> Self {
        Self {
            rotation_axis: Vec3::right(),
            rotation_speed: 0.15,
        }
    }
}

impl Script for TestRotationScript {
    fn start(&mut self, _data: ScriptStartEndData) -> RisResult<()> {
        Ok(())
    }

    fn update(&mut self, data: ScriptUpdateData) -> RisResult<()> {
        let ScriptUpdateData {
            game_object,
            frame,
            state: ris_data::god_state::GodState { scene, .. },
        } = data;

        let rotation = game_object.local_rotation(scene)?;
        let speed = self.rotation_speed * frame.average_seconds();
        let angle = 2.0 * PI * speed;
        let q = Quat::angle_axis(angle, self.rotation_axis);
        let new_rotation = q * rotation;
        game_object.set_local_rotation(scene, new_rotation)?;

        Ok(())
    }

    fn end(&mut self, _data: ScriptStartEndData) -> RisResult<()> {
        Ok(())
    }

    fn inspect(&mut self, data: ScriptInspectData) -> RisResult<()> {
        let ScriptInspectData { id, .. } = data;

        ris_core::inspector_util::drag_vec3(
            format!("rotation axis##{}", id),
            &mut self.rotation_axis,
        )?;

        ris_core::inspector_util::drag(format!("rotation axis##{}", id), &mut self.rotation_speed)?;

        Ok(())
    }

    fn serialize(&mut self, f: &mut SceneWriter) -> RisResult<()> {
        ris_io::write_vec3(f, self.rotation_axis)?;
        ris_io::write_f32(f, self.rotation_speed)?;
        Ok(())
    }

    fn deserialize(&mut self, f: &mut SceneReader) -> RisResult<()> {
        self.rotation_axis = ris_io::read_vec3(f)?;
        self.rotation_speed = ris_io::read_f32(f)?;
        Ok(())
    }
}

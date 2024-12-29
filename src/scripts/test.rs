use ris_data::ecs::script_prelude::*;
use ris_math::quaternion::Quat;
use ris_math::vector::Vec3;

#[derive(Debug, Default)]
pub struct Rotation {
    pub rotation_axis: Vec3,
}

impl Script for Rotation {
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
        let angle = frame.average_seconds();
        let q = Quat::angle_axis(angle, self.rotation_axis);
        let new_rotation = q * rotation;
        game_object.set_local_rotation(scene, new_rotation)?;

        Ok(())
    }

    fn end(&mut self, _data: ScriptStartEndData) -> RisResult<()> {
        Ok(())
    }

    fn inspect(&mut self, data: ScriptInspectData) -> RisResult<()> {
        let ScriptInspectData { id, ui, .. } = data;

        ui.label_text("label", "this is the script inspector");

        ris_core::ui_helper::util::drag_vec3(
            format!("rotation axis##{}", id),
            &mut self.rotation_axis,
        )?;

        Ok(())
    }

    fn serialize(&mut self, f: &mut SceneWriter) -> RisResult<()> {
        ris_io::write_vec3(f, self.rotation_axis)?;
        Ok(())
    }

    fn deserialize(&mut self, f: &mut SceneReader) -> RisResult<()> {
        self.rotation_axis = ris_io::read_vec3(f)?;
        Ok(())
    }

}

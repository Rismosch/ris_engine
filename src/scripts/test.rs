use ris_data::ecs::script_prelude::*;
use ris_math::quaternion::Quat;
use ris_math::vector::Vec3;

#[derive(Debug, Default)]
pub struct Rotation {
    pub rotation_axis: Vec3,
}

impl ISerializable for Rotation {
    fn serialize(&self) -> RisResult<Vec<u8>> {
        ris_error::new_result!("not implemented")
    }

    fn deserialize(_bytes: &[u8]) -> RisResult<Self> {
        ris_error::new_result!("not implemented")
    }
}

impl Script for Rotation {
    fn id() -> Sid {
        ris_debug::fsid!()
    }

    fn name(&self) -> &'static str {
        "TestRotation"
    }

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
        let ScriptInspectData { ui, .. } = data;

        ui.label_text("this is the script inspector", "label");

        ris_core::ui_helper::util::drag_vec3("rotation axis", &mut self.rotation_axis)?;

        Ok(())
    }
}

use ris_error::RisResult;

use crate::ui_helper::IUiHelperModule;
use crate::ui_helper::SharedStateWeakPtr;
use crate::ui_helper::UiHelperDrawData;
use crate::ui_helper::Selected;

pub struct InspectorModule {
    shared_state: SharedStateWeakPtr,
}

impl IUiHelperModule for InspectorModule {
    fn name() -> &'static str {
        "inspector"
    }

    fn build(shared_state: SharedStateWeakPtr) -> Box<dyn IUiHelperModule> {
        Box::new(Self{
            shared_state
        })
    }

    fn draw(&mut self, data: &mut UiHelperDrawData) -> RisResult<()> {
        let Some(selected) = self.shared_state.borrow().selected.clone() else {
            data.ui.label_text("##nothing_selected", "nothing selected");
            return Ok(());
        };

        match selected {
            Selected::GameObject(handle) => {
                if !handle.is_alive(&data.state.scene) {
                    self.shared_state.borrow_mut().selected = None;
                    return Ok(());
                }

                let mut name = handle.name(&data.state.scene)?;
                if data.ui.input_text("name", &mut name).build() {
                    handle.set_name(&data.state.scene, name)?;
                }

                let local_position = handle.local_position(&data.state.scene)?;
                let mut local_position_array: [f32; 3] = local_position.into();
                if data.ui.input_float3("local position", &mut local_position_array).build() {
                    ris_log::debug!("position");
                    handle.set_local_position(&data.state.scene, local_position.into())?;
                }

                let local_rotation = handle.local_rotation(&data.state.scene)?;
                let mut rotation_euler: [f32; 3] = ris_math::euler_angles::from(local_rotation).into();
                if data.ui.input_float3("local rotation", &mut rotation_euler).build() {
                    let new_rotation = ris_math::euler_angles::to_quat(rotation_euler.into());
                    handle.set_local_rotation(&data.state.scene, new_rotation)?;
                }

                let mut local_scale = handle.local_scale(&data.state.scene)?;
                if data.ui.input_float("local scale", &mut local_scale).build() {
                    handle.set_local_scale(&data.state.scene, local_scale)?;
                }

                ris_debug::gizmo::view_point(local_position, local_rotation, None)?;
            },
        }

        Ok(())
    }
}

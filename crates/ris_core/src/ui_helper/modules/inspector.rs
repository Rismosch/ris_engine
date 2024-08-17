use std::ffi::CString;

use ris_error::RisResult;
use ris_math::vector::Vec3;

use crate::ui_helper::IUiHelperModule;
use crate::ui_helper::selection::Selection;
use crate::ui_helper::selection::Selector;
use crate::ui_helper::SharedStateWeakPtr;
use crate::ui_helper::UiHelperDrawData;

pub struct InspectorModule {
    shared_state: SharedStateWeakPtr,
    cashed_euler_angles: Vec3,
}

impl IUiHelperModule for InspectorModule {
    fn name() -> &'static str {
        "inspector"
    }

    fn build(shared_state: SharedStateWeakPtr) -> Box<dyn IUiHelperModule> {
        Box::new(Self{
            shared_state,
            cashed_euler_angles: Vec3::default(),
        })
    }

    fn draw(&mut self, data: &mut UiHelperDrawData) -> RisResult<()> {
        let Some(selected) = self.shared_state.borrow().selector.get_selection() else {
            data.ui.label_text("##nothing_selected", "nothing selected");
            return Ok(());
        };

        match selected {
            Selection::GameObject(handle) => {
                if !handle.is_alive(&data.state.scene) {
                    self.shared_state.borrow_mut().selector.set_selection(None);
                    return Ok(());
                }

                if self.shared_state.borrow().selector.selection_changed() {
                    let rotation = handle.local_rotation(&data.state.scene)?;
                    let mut e = ris_math::euler_angles::from(rotation);

                    // get rid of negative zero, caused by euler conversion
                    let tolerance = 0.000_000_1;
                    if f32::abs(e.x()) < tolerance {
                        e.set_x(0.0);
                    }

                    if f32::abs(e.y()) < tolerance {
                        e.set_y(0.0);
                    }

                    if f32::abs(e.z()) < tolerance {
                        e.set_z(0.0);
                    }

                    ris_log::trace!("cached euler angles: {:?}", self.cashed_euler_angles);
                }

                let mut name = handle.name(&data.state.scene)?;
                if data.ui.input_text("name", &mut name).build() {
                    handle.set_name(&data.state.scene, name)?;
                }

                let format = CString::new("%.3f")?;

                let label = CString::new("position")?;
                let position = handle.local_position(&data.state.scene)?;
                let mut position: [f32; 3] = position.into();
                let focused = unsafe {imgui::sys::igDragFloat3(
                    label.as_ptr(),
                    position.as_mut_ptr(),
                    0.01,
                    0.0,
                    0.0,
                    format.as_ptr(),
                    0,
                )};
                if focused {
                    handle.set_local_position(&data.state.scene, position.into())?;
                }

                let label = CString::new("rotation")?;
                let mut euler_angles: [f32; 3] = self.cashed_euler_angles.into();
                let focused = unsafe {imgui::sys::igDragFloat3(
                    label.as_ptr(),
                    euler_angles.as_mut_ptr(),
                    1.0,
                    0.0,
                    0.0,
                    format.as_ptr(),
                    0,
                )};
                if focused {
                    self.cashed_euler_angles = euler_angles.into();
                    let new_rotation = ris_math::euler_angles::to_quat(self.cashed_euler_angles);
                    handle.set_local_rotation(&data.state.scene, new_rotation)?;
                }

                let label = CString::new("scale")?;
                let mut scale = handle.local_scale(&data.state.scene)?;
                let focused = unsafe {imgui::sys::igDragFloat(
                    label.as_ptr(),
                    &mut scale as *mut f32,
                    0.001,
                    0.001,
                    f32::MAX,
                    format.as_ptr(),
                    0,
                )};
                if focused {
                    handle.set_local_scale(&data.state.scene, scale)?;
                }
            },
        }

        Ok(())
    }
}

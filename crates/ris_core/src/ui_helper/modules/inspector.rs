use std::ffi::CString;

use ris_data::game_object::scene::Scene;
use ris_data::game_object::scene::SceneResult;
use ris_data::game_object::GameObjectHandle;
use ris_error::RisResult;
use ris_math::quaternion::Quat;
use ris_math::vector::Vec3;

use crate::ui_helper::selection::Selection;
use crate::ui_helper::IUiHelperModule;
use crate::ui_helper::SharedStateWeakPtr;
use crate::ui_helper::UiHelperDrawData;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Space {
    Local,
    World,
}

pub struct InspectorModule {
    shared_state: SharedStateWeakPtr,
    cashed_euler_angles: Vec3,
    space: Space,
}

impl IUiHelperModule for InspectorModule {
    fn name() -> &'static str {
        "inspector"
    }

    fn build(shared_state: SharedStateWeakPtr) -> Box<dyn IUiHelperModule> {
        Box::new(Self {
            shared_state,
            cashed_euler_angles: Vec3::default(),
            space: Space::Local,
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

                let get_position:Box<dyn Fn(GameObjectHandle, &Scene) -> SceneResult<Vec3>>;
                let set_position:Box<dyn Fn(GameObjectHandle, &Scene, Vec3) -> SceneResult<()>>;
                let get_rotation:Box<dyn Fn(GameObjectHandle, &Scene) -> SceneResult<Quat>>;
                let set_rotation:Box<dyn Fn(GameObjectHandle, &Scene, Quat) -> SceneResult<()>>;
                let get_scale:Box<dyn Fn(GameObjectHandle, &Scene) -> SceneResult<f32>>;
                let set_scale:Box<dyn Fn(GameObjectHandle, &Scene, f32) -> SceneResult<()>>;

                match self.space {
                    Space::Local => {
                        get_position = Box::new(GameObjectHandle::local_position);
                        set_position = Box::new(GameObjectHandle::set_local_position);
                        get_rotation = Box::new(GameObjectHandle::local_rotation);
                        set_rotation = Box::new(GameObjectHandle::set_local_rotation);
                        get_scale = Box::new(GameObjectHandle::local_scale);
                        set_scale = Box::new(GameObjectHandle::set_local_scale);
                    },
                    Space::World => {
                        get_position = Box::new(GameObjectHandle::world_position);
                        set_position = Box::new(GameObjectHandle::set_world_position);
                        get_rotation = Box::new(GameObjectHandle::world_rotation);
                        set_rotation = Box::new(GameObjectHandle::set_world_rotation);
                        get_scale = Box::new(GameObjectHandle::world_scale);
                        set_scale = Box::new(GameObjectHandle::set_world_scale);
                    },
                };

                let mut name = handle.name(&data.state.scene)?;
                if data.ui.input_text("name", &mut name).build() {
                    handle.set_name(&data.state.scene, name)?;
                }

                data.ui.separator();

                let space_items = ["Local", "World"];
                let mut current_space_item = match self.space {
                    Space::Local => 0,
                    Space::World => 1,
                };
                let space_changed =
                    data.ui
                        .combo_simple_string("transform", &mut current_space_item, &space_items);
                match current_space_item {
                    0 => self.space = Space::Local,
                    1 => self.space = Space::World,
                    _ => unreachable!(),
                }

                // transforming a quaternion back and forth to euler angles produces artifacts.
                // (did i mention euler angles suck?) we cache them to avoid these artifacts
                let selection_changed = self.shared_state.borrow().selector.selection_changed();
                if space_changed || selection_changed {
                    let rotation = get_rotation(handle, &data.state.scene)?;
                    self.cashed_euler_angles = ris_math::euler_angles::from(rotation);
                }

                let format = CString::new("%.3f")?;

                let label = CString::new("position")?;
                let position = get_position(handle, &data.state.scene)?;
                let mut position: [f32; 3] = purge_negative_0(position.into());
                let focused = unsafe {
                    imgui::sys::igDragFloat3(
                        label.as_ptr(),
                        position.as_mut_ptr(),
                        0.01,
                        0.0,
                        0.0,
                        format.as_ptr(),
                        0,
                    )
                };
                if focused {
                    set_position(handle, &data.state.scene, position.into())?;
                }

                let label = CString::new("rotation")?;
                let mut euler_angles: [f32; 3] = purge_negative_0(self.cashed_euler_angles.into());
                let focused = unsafe {
                    imgui::sys::igDragFloat3(
                        label.as_ptr(),
                        euler_angles.as_mut_ptr(),
                        1.0,
                        0.0,
                        0.0,
                        format.as_ptr(),
                        0,
                    )
                };
                if focused {
                    self.cashed_euler_angles = euler_angles.into();
                    let new_rotation = ris_math::euler_angles::to_quat(self.cashed_euler_angles);
                    set_rotation(handle, &data.state.scene, new_rotation)?;
                }

                let label = CString::new("scale")?;
                let scale_min = 0.001;
                let mut scale = get_scale(handle, &data.state.scene)?;
                let focused = unsafe {
                    imgui::sys::igDragFloat(
                        label.as_ptr(),
                        &mut scale as *mut f32,
                        0.001,
                        scale_min,
                        f32::MAX,
                        format.as_ptr(),
                        0,
                    )
                };
                scale = f32::max(scale, scale_min);
                if focused {
                    set_scale(handle, &data.state.scene, scale)?;
                }

                data.ui.separator();
            }
        }

        Ok(())
    }
}

fn purge_negative_0(mut value: [f32; 3]) -> [f32; 3] {
    let tolerance = 0.000_000_1;

    for item in value.iter_mut() {
        if item.abs() < tolerance {
            *item = 0.0;
        }
    }

    value
}

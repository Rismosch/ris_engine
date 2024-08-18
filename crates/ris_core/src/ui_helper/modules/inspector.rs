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
    space: Space,
    cached_vector_xyz: Vec3,
    cached_vector_xyw: Vec3,
    cached_vector_xzw: Vec3,
    cached_vector_yzw: Vec3,
}

impl IUiHelperModule for InspectorModule {
    fn name() -> &'static str {
        "inspector"
    }

    fn build(shared_state: SharedStateWeakPtr) -> Box<dyn IUiHelperModule> {
        Box::new(Self {
            shared_state,
            space: Space::Local,
            cached_vector_yzw: Vec3(0.0, 0.0, 1.0),
            cached_vector_xzw: Vec3(0.0, 0.0, 1.0),
            cached_vector_xyw: Vec3(0.0, 0.0, 1.0),
            cached_vector_xyz: Vec3(1.0, 0.0, 0.0),
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

                let mut name = handle.name(&data.state.scene)?;
                if data.ui.input_text("name", &mut name).build() {
                    handle.set_name(&data.state.scene, name)?;
                }

                let mut is_visible = handle.is_visible(&data.state.scene)?;
                if data.ui.checkbox("is visible", &mut is_visible) {
                    handle.set_visible(&data.state.scene, is_visible)?;
                }

                {
                    let _token = data.ui.begin_disabled(true);
                    let mut is_visible_in_hierarchy = handle.is_visible_in_hierarchy(&data.state.scene)?;
                    data.ui.checkbox("is visible in hierarchy", &mut is_visible_in_hierarchy);
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

                let get_position: fn(GameObjectHandle, &Scene) -> SceneResult<Vec3>;
                let set_position: fn(GameObjectHandle, &Scene, Vec3) -> SceneResult<()>;
                let get_rotation: fn(GameObjectHandle, &Scene) -> SceneResult<Quat>;
                let set_rotation: fn(GameObjectHandle, &Scene, Quat) -> SceneResult<()>;
                let get_scale: fn(GameObjectHandle, &Scene) -> SceneResult<f32>;
                let set_scale: fn(GameObjectHandle, &Scene, f32) -> SceneResult<()>;

                match self.space {
                    Space::Local => {
                        get_position = GameObjectHandle::local_position;
                        set_position = GameObjectHandle::set_local_position;
                        get_rotation = GameObjectHandle::local_rotation;
                        set_rotation = GameObjectHandle::set_local_rotation;
                        get_scale = GameObjectHandle::local_scale;
                        set_scale = GameObjectHandle::set_local_scale;
                    },
                    Space::World => {
                        get_position = GameObjectHandle::world_position;
                        set_position = GameObjectHandle::set_world_position;
                        get_rotation = GameObjectHandle::world_rotation;
                        set_rotation = GameObjectHandle::set_world_rotation;
                        get_scale = GameObjectHandle::world_scale;
                        set_scale = GameObjectHandle::set_world_scale;
                    },
                };

                let selection_changed = self.shared_state.borrow().selector.selection_changed();
                if selection_changed || space_changed {
                    let rotation = get_rotation(handle, &data.state.scene)?;
                    self.cache_rotation(rotation);
                }

                let format = CString::new("%.3f")?;

                let label = CString::new("position")?;
                let position = get_position(handle, &data.state.scene)?;
                let mut position: [f32; 3] = position.into();
                purge_negative_0(&mut position);
                let changed = unsafe {
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
                if changed {
                    set_position(handle, &data.state.scene, position.into())?;
                }

                let label = CString::new("rotation")?;
                let rotation = get_rotation(handle, &data.state.scene)?;
                let mut old_rotation: [f32; 4] = rotation.into();
                purge_negative_0(&mut old_rotation);
                let mut new_rotation: [f32; 4] = old_rotation.clone();
                let changed = unsafe {
                    imgui::sys::igDragFloat4(
                        label.as_ptr(),
                        new_rotation.as_mut_ptr(),
                        0.001,
                        -1.0,
                        1.0,
                        format.as_ptr(),
                        0,
                    )
                };
                if changed {
                    let x: f32;
                    let y: f32;
                    let z: f32;
                    let w: f32;

                    if new_rotation[0] != old_rotation[0] {
                        x = new_rotation[0].clamp(-1.0, 1.0);
                        let radius_yzw = f32::sqrt(1.0 - x * x);
                        let scaled_yzw = radius_yzw * self.cached_vector_yzw.normalize();
                        y = scaled_yzw.0;
                        z = scaled_yzw.1;
                        w = scaled_yzw.2;
                        self.cached_vector_xyz = Vec3(x, y, z);
                        self.cached_vector_xyw = Vec3(x, y, w);
                        self.cached_vector_xzw = Vec3(x, z, w);
                    } else if new_rotation[1] != old_rotation[1] {
                        y = new_rotation[1].clamp(-1.0, 1.0);
                        let radius_xzw = f32::sqrt(1.0 - y * y);
                        let scaled_xzw = radius_xzw * self.cached_vector_xzw.normalize();
                        x = scaled_xzw.0;
                        z = scaled_xzw.1;
                        w = scaled_xzw.2;
                        self.cached_vector_xyz = Vec3(x, y, z);
                        self.cached_vector_xyw = Vec3(x, y, w);
                        self.cached_vector_yzw = Vec3(y, z, w);
                    } else if new_rotation[2] != old_rotation[2] {
                        z = new_rotation[2].clamp(-1.0, 1.0);
                        let radius_xyw = f32::sqrt(1.0 - z * z);
                        let scaled_xyw = radius_xyw * self.cached_vector_xyw.normalize();
                        x = scaled_xyw.0;
                        y = scaled_xyw.1;
                        w = scaled_xyw.2;
                        self.cached_vector_xyz = Vec3(x, y, z);
                        self.cached_vector_yzw = Vec3(y, z, w);
                        self.cached_vector_xzw = Vec3(x, z, w);
                    } else if new_rotation[3] != old_rotation[3] {
                        w = new_rotation[3].clamp(-1.0, 1.0);
                        let radius_xyz = f32::sqrt(1.0 - w * w);
                        let scaled_xyz = radius_xyz * self.cached_vector_xyz.normalize();
                        x = scaled_xyz.0;
                        y = scaled_xyz.1;
                        z = scaled_xyz.2;
                        self.cached_vector_xyw = Vec3(x, y, w);
                        self.cached_vector_xzw = Vec3(x, z, w);
                        self.cached_vector_yzw = Vec3(y, z, w);
                    } else {
                        x = new_rotation[0].clamp(-1.0, 1.0);
                        y = new_rotation[1].clamp(-1.0, 1.0);
                        z = new_rotation[2].clamp(-1.0, 1.0);
                        w = new_rotation[3].clamp(-1.0, 1.0);
                    }

                    let quat = if x.is_nan() || x.is_infinite() ||
                        y.is_nan() || y.is_infinite() ||
                        z.is_nan() || z.is_infinite() ||
                        w.is_nan() || w.is_infinite() {
                        ris_log::error!("the fuck {} {} {} {}", x, y, z, w);
                        let q = Quat::identity();
                        self.cache_rotation(q);
                        q
                    } else {
                        Quat(x, y, z, w).normalize()
                    };

                    let q1 = quat;
                    set_rotation(handle, &data.state.scene, quat)?;
                    let q2 = get_rotation(handle, &data.state.scene)?;
                    ris_log::trace!("hi {:?} {:?}", q1, q2);
                }

                data.ui.same_line();
                let set_rotation_popup_id = "set_rotation_popup";
                if data.ui.button("set") {
                    data.ui.open_popup(set_rotation_popup_id)
                }

                if let Some(_token) = data.ui.begin_popup(set_rotation_popup_id) {
                    let mut rotation = None;

                    if data.ui.menu_item("right up") {
                        rotation = Some(Quat::look_at(Vec3::right(), Vec3::up()));
                    }

                    if data.ui.menu_item("right forward") {
                        rotation = Some(Quat::look_at(Vec3::right(), Vec3::forward()));
                    }

                    if data.ui.menu_item("right down") {
                        rotation = Some(Quat::look_at(Vec3::right(), Vec3::down()));
                    }

                    if data.ui.menu_item("right backward") {
                        rotation = Some(Quat::look_at(Vec3::right(), Vec3::backward()));
                    }

                    data.ui.separator();

                    if data.ui.menu_item("left up") {
                        rotation = Some(Quat::look_at(Vec3::left(), Vec3::up()));
                    }

                    if data.ui.menu_item("left forward") {
                        rotation = Some(Quat::look_at(Vec3::left(), Vec3::forward()));
                    }

                    if data.ui.menu_item("left down") {
                        rotation = Some(Quat::look_at(Vec3::left(), Vec3::down()));
                    }

                    if data.ui.menu_item("left backward") {
                        rotation = Some(Quat::look_at(Vec3::left(), Vec3::backward()));
                    }

                    data.ui.separator();

                    if data.ui.menu_item("forward up") {
                        rotation = Some(Quat::look_at(Vec3::forward(), Vec3::up()));
                    }

                    if data.ui.menu_item("forward right") {
                        rotation = Some(Quat::look_at(Vec3::forward(), Vec3::right()));
                    }

                    if data.ui.menu_item("forward down") {
                        rotation = Some(Quat::look_at(Vec3::forward(), Vec3::down()));
                    }

                    if data.ui.menu_item("forward left") {
                        rotation = Some(Quat::look_at(Vec3::forward(), Vec3::left()));
                    }

                    data.ui.separator();

                    if data.ui.menu_item("backward up") {
                        rotation = Some(Quat::look_at(Vec3::backward(), Vec3::up()));
                    }

                    if data.ui.menu_item("backward right") {
                        rotation = Some(Quat::look_at(Vec3::backward(), Vec3::right()));
                    }

                    if data.ui.menu_item("backward down") {
                        rotation = Some(Quat::look_at(Vec3::backward(), Vec3::down()));
                    }

                    if data.ui.menu_item("backward left") {
                        rotation = Some(Quat::look_at(Vec3::backward(), Vec3::left()));
                    }

                    data.ui.separator();

                    if data.ui.menu_item("up forward") {
                        rotation = Some(Quat::look_at(Vec3::up(), Vec3::forward()));
                    }

                    if data.ui.menu_item("up right") {
                        rotation = Some(Quat::look_at(Vec3::up(), Vec3::right()));
                    }

                    if data.ui.menu_item("up backward") {
                        rotation = Some(Quat::look_at(Vec3::up(), Vec3::backward()));
                    }

                    if data.ui.menu_item("up left") {
                        rotation = Some(Quat::look_at(Vec3::up(), Vec3::left()));
                    }

                    data.ui.separator();

                    if data.ui.menu_item("down forward") {
                        rotation = Some(Quat::look_at(Vec3::down(), Vec3::forward()));
                    }

                    if data.ui.menu_item("down right") {
                        rotation = Some(Quat::look_at(Vec3::down(), Vec3::right()));
                    }

                    if data.ui.menu_item("down backward") {
                        rotation = Some(Quat::look_at(Vec3::down(), Vec3::backward()));
                    }

                    if data.ui.menu_item("down left") {
                        rotation = Some(Quat::look_at(Vec3::down(), Vec3::left()));
                    }

                    if let Some(rotation) = rotation {
                        set_rotation(handle, &data.state.scene, rotation)?;
                        self.cache_rotation(rotation);
                    }
                }

                let label = CString::new("scale")?;
                let scale_min = 0.001;
                let mut scale = get_scale(handle, &data.state.scene)?;
                let changed = unsafe {
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
                if changed {
                    set_scale(handle, &data.state.scene, scale)?;
                }

                let world_position = handle.world_position(&data.state.scene)?;
                let world_rotation = handle.world_rotation(&data.state.scene)?;
                let local_rotation = handle.local_rotation(&data.state.scene)?;
                let parent_world_rotation = match handle.parent(&data.state.scene)? {
                    Some(parent) => parent.world_rotation(&data.state.scene)?,
                    None => Quat::identity(),
                };
                let (_, axis) = local_rotation.into();
                let mut rotated_axis = parent_world_rotation.rotate(axis);
                rotated_axis *= 0.5;
                ris_debug::gizmo::view_point(
                    world_position,
                    world_rotation,
                    None,
                )?;
                ris_debug::gizmo::segment(
                    world_position - rotated_axis,
                    world_position + rotated_axis,
                    ris_math::color::Rgb::white(),
                )?;

                data.ui.separator();
            }
        }

        Ok(())
    }
}

impl InspectorModule {
    fn cache_rotation(&mut self, q: Quat) {
        self.cached_vector_xyz = Vec3(q.x(), q.y(), q.z());
        self.cached_vector_xyw = Vec3(q.x(), q.y(), q.w());
        self.cached_vector_xzw = Vec3(q.x(), q.z(), q.w());
        self.cached_vector_yzw = Vec3(q.y(), q.z(), q.w());

        let tolerance = 0.000_01;
        if self.cached_vector_xyz.length_squared() < tolerance {
            self.cached_vector_xyz = Vec3(1.0, 0.0, 0.0);
        }
        if self.cached_vector_xyw.length_squared() < tolerance {
            self.cached_vector_xyw = Vec3(0.0, 0.0, 1.0);
        }
        if self.cached_vector_xzw.length_squared() < tolerance {
            self.cached_vector_xzw = Vec3(0.0, 0.0, 1.0);
        }
        if self.cached_vector_yzw.length_squared() < tolerance {
            self.cached_vector_yzw = Vec3(0.0, 0.0, 1.0);
        }
    }
}

fn purge_negative_0(value: &mut [f32]) {
    let tolerance = 0.000_01;

    for item in value.iter_mut() {
        if item.abs() < tolerance {
            *item = 0.0;
        }
    }
}

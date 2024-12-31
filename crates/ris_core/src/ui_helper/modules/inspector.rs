use std::any::TypeId;
use std::ffi::CString;

use imgui::Ui;

use ris_asset::asset_loader::LoadError;
use ris_data::asset_id::AssetId;
use ris_data::ecs::components::mesh_renderer::MeshRendererComponent;
use ris_data::ecs::components::script::DynScriptComponent;
use ris_data::ecs::components::script::ScriptInspectData;
use ris_data::ecs::decl::GameObjectHandle;
use ris_data::ecs::error::EcsResult;
use ris_data::ecs::scene::Scene;
use ris_error::Extensions;
use ris_error::RisResult;
use ris_jobs::job_future::JobFuture;
use ris_math::quaternion::Quat;
use ris_math::vector::Vec3;

use crate::ui_helper::selection::Selection;
use crate::ui_helper::util;
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

    // game object
    space: Space,
    cached_rotation: Quat,
    cached_xyz: Vec3,
    cached_xyw: Vec3,
    cached_xzw: Vec3,
    cached_yzw: Vec3,
    component_filter: String,

    // asset
    load_asset_jobs: Vec<JobFuture<Result<Vec<u8>, LoadError>>>,
    loaded_asset: Vec<u8>,
}

impl IUiHelperModule for InspectorModule {
    fn name() -> &'static str {
        "inspector"
    }

    fn build(shared_state: SharedStateWeakPtr) -> Box<dyn IUiHelperModule> {
        Box::new(Self {
            shared_state,

            // game object
            space: Space::Local,
            cached_rotation: Quat::identity(),
            cached_yzw: Vec3(0.0, 0.0, 1.0),
            cached_xzw: Vec3(0.0, 0.0, 1.0),
            cached_xyw: Vec3(0.0, 0.0, 1.0),
            cached_xyz: Vec3(1.0, 0.0, 0.0),
            component_filter: String::new(),

            // asset
            load_asset_jobs: Vec::new(),
            loaded_asset: Vec::new(),
        })
    }

    fn draw(&mut self, data: &mut UiHelperDrawData) -> RisResult<()> {
        let Some(selection) = self.shared_state.borrow().selector.get_selection() else {
            data.ui.label_text("##nothing_selected", "nothing selected");
            return Ok(());
        };

        match selection {
            Selection::GameObject(game_object) => {
                if !game_object.is_alive(&data.state.scene) {
                    self.shared_state.borrow_mut().selector.set_selection(None);
                    return Ok(());
                }

                let mut name = game_object.name(&data.state.scene)?;
                if data.ui.input_text("name", &mut name).build() {
                    game_object.set_name(&data.state.scene, name)?;
                }

                let mut is_active = game_object.is_active(&data.state.scene)?;
                if data.ui.checkbox("is active", &mut is_active) {
                    game_object.set_active(&data.state.scene, is_active)?;
                }

                {
                    let _token = data.ui.begin_disabled(true);
                    let mut is_active_in_hierarchy =
                        game_object.is_active_in_hierarchy(&data.state.scene)?;
                    data.ui
                        .checkbox("is active in hierarchy", &mut is_active_in_hierarchy);
                }

                data.ui.separator();

                let space_items = ["Local", "World"];
                let mut current_space_item = match self.space {
                    Space::Local => 0,
                    Space::World => 1,
                };
                data.ui
                    .combo_simple_string("transform", &mut current_space_item, &space_items);
                match current_space_item {
                    0 => self.space = Space::Local,
                    1 => self.space = Space::World,
                    _ => unreachable!(),
                }

                let get_position: fn(GameObjectHandle, &Scene) -> EcsResult<Vec3>;
                let set_position: fn(GameObjectHandle, &Scene, Vec3) -> EcsResult<()>;
                let get_rotation: fn(GameObjectHandle, &Scene) -> EcsResult<Quat>;
                let set_rotation: fn(GameObjectHandle, &Scene, Quat) -> EcsResult<()>;
                let get_scale: fn(GameObjectHandle, &Scene) -> EcsResult<f32>;
                let set_scale: fn(GameObjectHandle, &Scene, f32) -> EcsResult<()>;

                match self.space {
                    Space::Local => {
                        get_position = GameObjectHandle::local_position;
                        set_position = GameObjectHandle::set_local_position;
                        get_rotation = GameObjectHandle::local_rotation;
                        set_rotation = GameObjectHandle::set_local_rotation;
                        get_scale = GameObjectHandle::local_scale;
                        set_scale = GameObjectHandle::set_local_scale;
                    }
                    Space::World => {
                        get_position = GameObjectHandle::world_position;
                        set_position = GameObjectHandle::set_world_position;
                        get_rotation = GameObjectHandle::world_rotation;
                        set_rotation = GameObjectHandle::set_world_rotation;
                        get_scale = GameObjectHandle::world_scale;
                        set_scale = GameObjectHandle::set_world_scale;
                    }
                };

                let mut position = get_position(game_object, &data.state.scene)?;

                let changed = util::drag_vec3("position", &mut position)?;
                if changed {
                    set_position(game_object, &data.state.scene, position)?;
                }

                let format = CString::new("%.3f")?;
                let label = CString::new("rotation")?;
                if !data.ui.is_mouse_dragging(imgui::MouseButton::Left) {
                    let rotation = get_rotation(game_object, &data.state.scene)?;
                    self.cached_rotation = rotation;
                    self.cache_rotation_axes(rotation);
                }
                let mut old_rotation: [f32; 4] = self.cached_rotation.into();
                util::purge_negative_0(&mut old_rotation);
                let mut new_rotation: [f32; 4] = old_rotation;
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
                self.cached_rotation = new_rotation.into();
                if changed {
                    let x: f32;
                    let y: f32;
                    let z: f32;
                    let w: f32;

                    if new_rotation[0] != old_rotation[0] {
                        x = new_rotation[0].clamp(-1.0, 1.0);
                        let radius_yzw = f32::sqrt(1.0 - x * x);
                        let scaled_yzw = radius_yzw * self.cached_yzw.normalize();
                        y = scaled_yzw.0;
                        z = scaled_yzw.1;
                        w = scaled_yzw.2;
                        self.cached_rotation.set_y(y);
                        self.cached_rotation.set_z(z);
                        self.cached_rotation.set_w(w);
                        self.cached_xyz = Vec3(x, y, z);
                        self.cached_xyw = Vec3(x, y, w);
                        self.cached_xzw = Vec3(x, z, w);
                    } else if new_rotation[1] != old_rotation[1] {
                        y = new_rotation[1].clamp(-1.0, 1.0);
                        let radius_xzw = f32::sqrt(1.0 - y * y);
                        let scaled_xzw = radius_xzw * self.cached_xzw.normalize();
                        x = scaled_xzw.0;
                        z = scaled_xzw.1;
                        w = scaled_xzw.2;
                        self.cached_rotation.set_x(x);
                        self.cached_rotation.set_z(z);
                        self.cached_rotation.set_w(w);
                        self.cached_xyz = Vec3(x, y, z);
                        self.cached_xyw = Vec3(x, y, w);
                        self.cached_yzw = Vec3(y, z, w);
                    } else if new_rotation[2] != old_rotation[2] {
                        z = new_rotation[2].clamp(-1.0, 1.0);
                        let radius_xyw = f32::sqrt(1.0 - z * z);
                        let scaled_xyw = radius_xyw * self.cached_xyw.normalize();
                        x = scaled_xyw.0;
                        y = scaled_xyw.1;
                        w = scaled_xyw.2;
                        self.cached_rotation.set_x(x);
                        self.cached_rotation.set_y(y);
                        self.cached_rotation.set_w(w);
                        self.cached_xyz = Vec3(x, y, z);
                        self.cached_yzw = Vec3(y, z, w);
                        self.cached_xzw = Vec3(x, z, w);
                    } else if new_rotation[3] != old_rotation[3] {
                        w = new_rotation[3].clamp(-1.0, 1.0);
                        let radius_xyz = f32::sqrt(1.0 - w * w);
                        let scaled_xyz = radius_xyz * self.cached_xyz.normalize();
                        x = scaled_xyz.0;
                        y = scaled_xyz.1;
                        z = scaled_xyz.2;
                        self.cached_rotation.set_x(x);
                        self.cached_rotation.set_y(y);
                        self.cached_rotation.set_z(z);
                        self.cached_xyw = Vec3(x, y, w);
                        self.cached_xzw = Vec3(x, z, w);
                        self.cached_yzw = Vec3(y, z, w);
                    } else {
                        x = new_rotation[0].clamp(-1.0, 1.0);
                        y = new_rotation[1].clamp(-1.0, 1.0);
                        z = new_rotation[2].clamp(-1.0, 1.0);
                        w = new_rotation[3].clamp(-1.0, 1.0);
                    }

                    let q = if x.is_nan()
                        || x.is_infinite()
                        || y.is_nan()
                        || y.is_infinite()
                        || z.is_nan()
                        || z.is_infinite()
                        || w.is_nan()
                        || w.is_infinite()
                    {
                        let identity = Quat::identity();
                        self.cache_rotation_axes(identity);
                        identity
                    } else {
                        Quat(x, y, z, w).normalize()
                    };

                    set_rotation(game_object, &data.state.scene, q)?;
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
                        set_rotation(game_object, &data.state.scene, rotation)?;
                        self.cache_rotation_axes(rotation);
                    }
                }

                let label = CString::new("scale")?;
                let scale_min = 0.001;
                let mut scale = get_scale(game_object, &data.state.scene)?;
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
                    set_scale(game_object, &data.state.scene, scale)?;
                }

                let world_position = game_object.world_position(&data.state.scene)?;
                let world_rotation = game_object.world_rotation(&data.state.scene)?;

                let rotation_axis = match self.space {
                    Space::Local => {
                        let axis_rotation = game_object.local_rotation(&data.state.scene)?;
                        let (_, axis) = axis_rotation.into();

                        match game_object.parent(&data.state.scene)? {
                            Some(parent) => parent.world_rotation(&data.state.scene)?.rotate(axis),
                            None => axis,
                        }
                    }
                    Space::World => {
                        let (_, axis) = world_rotation.into();
                        axis
                    }
                };

                ris_debug::gizmo::view_point(world_position, world_rotation, None)?;
                ris_debug::gizmo::segment(
                    world_position - rotation_axis * 0.5,
                    world_position + rotation_axis * 0.5,
                    ris_math::color::Rgb::white(),
                )?;

                data.ui.separator();
                data.ui.separator();
                data.ui.separator();

                let components = game_object.components(&data.state.scene)?;
                //data.ui.text(format!("{} components", components.len()));

                for component in components {
                    let index = component.scene_id().index;

                    let delete_requested;

                    if component.type_id() == TypeId::of::<MeshRendererComponent>() {
                        //let ptr = data.state.scene.mesh_renderer_components[index].to_weak();
                        //let aref_mut = ptr.borrow_mut();

                        let header =
                            ComponentHeader::draw(data.ui, format!("mesh##{:?}", component));
                        delete_requested = header.delete_requested;
                        if !header.is_open {
                            continue;
                        }

                        data.ui.text("im a mesh :)");
                    } else if component.type_id() == TypeId::of::<DynScriptComponent>() {
                        let ptr = data.state.scene.script_components[index].to_weak();
                        let mut aref_mut = ptr.borrow_mut();
                        let script_name = aref_mut.type_name().into_ris_error()?;

                        let game_object = aref_mut.game_object();
                        let script = aref_mut.script_mut().into_ris_error()?;

                        let header = ComponentHeader::draw(
                            data.ui,
                            format!("{} (script)##{:?}", script_name, component),
                        );
                        delete_requested = header.delete_requested;
                        if !header.is_open {
                            continue;
                        }

                        let script_inspect_data = ScriptInspectData {
                            id: format!("{:?}", component),
                            ui: data.ui,
                            game_object,
                            frame: data.frame,
                            state: data.state,
                        };

                        script.inspect(script_inspect_data)?;
                    } else {
                        let header = ComponentHeader::draw(
                            data.ui,
                            format!("{:?}##{:?}", component.type_id(), component),
                        );
                        delete_requested = header.delete_requested;
                    }

                    if delete_requested {
                        game_object.remove_and_destroy_component(&data.state.scene, component);
                    }
                }

                data.ui.separator();

                let add_component_popup_id = "add_component_popup_id";
                if data.ui.button("add component...") {
                    data.ui.open_popup(add_component_popup_id);
                }

                if let Some(_token) = data.ui.begin_popup(add_component_popup_id) {
                    data.ui
                        .input_text("filter", &mut self.component_filter)
                        .build();

                    for factory in data.state.scene.registry.component_factories() {
                        let name = factory.component_name();
                        if !name
                            .to_lowercase()
                            .contains(&self.component_filter.to_lowercase())
                        {
                            continue;
                        }

                        if data.ui.menu_item(name) {
                            factory.make(&data.state.scene, game_object)?;
                        }
                    }

                    data.ui.separator();

                    for factory in data.state.scene.registry.script_factories() {
                        let name = factory.script_name();
                        if !name
                            .to_lowercase()
                            .contains(&self.component_filter.to_lowercase())
                        {
                            continue;
                        }

                        if data.ui.menu_item(name) {
                            factory.make_and_attach(&data.state.scene, game_object)?;
                        }
                    }
                }
            }
            Selection::AssetPath(path_buf) => {
                let path_string = ris_io::path::to_str(&path_buf);
                data.ui.text(&path_string);
                let id = AssetId::Path(path_string.clone());

                let selection_changed = self.shared_state.borrow().selector.selection_changed();

                if selection_changed {
                    let mut actual_path = self.shared_state.borrow().app_info.asset_path()?;
                    actual_path.push(path_buf);
                    if !actual_path.is_dir() {
                        let job = ris_asset::load_async(id.clone());
                        self.load_asset_jobs.push(job);
                    }

                    self.loaded_asset.clear();
                }

                if !self.load_asset_jobs.is_empty() {
                    let job = self.load_asset_jobs.remove(0);
                    match job.try_take() {
                        Ok(data) => self.loaded_asset = data?,
                        Err(job) => self.load_asset_jobs.insert(0, job),
                    }
                }

                let size = self.loaded_asset.len();
                data.ui.text(format!("size: {:?}", size));
                let _disabled_token = data.ui.begin_disabled(size == 0);

                if path_string.ends_with(ris_asset::assets::ris_scene::EXTENSION)
                    && data.ui.button("load")
                {
                    let reserved =
                        ris_asset::assets::ris_scene::load(&data.state.scene, &self.loaded_asset)?;
                    if let Some(chunk_index) = reserved {
                        self.shared_state
                            .borrow_mut()
                            .set_chunk(chunk_index, Some(id));
                        ris_log::info!("loaded asset into chunk {}", chunk_index);
                    }
                }
            }
        }

        Ok(())
    }
}

impl InspectorModule {
    fn cache_rotation_axes(&mut self, q: Quat) {
        self.cached_xyz = Vec3(q.x(), q.y(), q.z());
        self.cached_xyw = Vec3(q.x(), q.y(), q.w());
        self.cached_xzw = Vec3(q.x(), q.z(), q.w());
        self.cached_yzw = Vec3(q.y(), q.z(), q.w());

        let tolerance = 0.000_01;
        if self.cached_xyz.length_squared() < tolerance {
            self.cached_xyz = Vec3(1.0, 0.0, 0.0);
        }
        if self.cached_xyw.length_squared() < tolerance {
            self.cached_xyw = Vec3(0.0, 0.0, 1.0);
        }
        if self.cached_xzw.length_squared() < tolerance {
            self.cached_xzw = Vec3(0.0, 0.0, 1.0);
        }
        if self.cached_yzw.length_squared() < tolerance {
            self.cached_yzw = Vec3(0.0, 0.0, 1.0);
        }
    }
}

struct ComponentHeader {
    is_open: bool,
    delete_requested: bool,
}

impl ComponentHeader {
    fn draw(ui: &Ui, label: impl AsRef<str>) -> Self {
        let header_flags = imgui::TreeNodeFlags::empty();
        let is_open = ui.collapsing_header(label, header_flags);

        let context_is_open = unsafe { imgui::sys::igBeginPopupContextItem(std::ptr::null(), 1) };

        let mut delete_requested = false;

        if context_is_open {
            if ui.button("delete") {
                delete_requested = true;
                unsafe { imgui::sys::igCloseCurrentPopup() };
            }

            unsafe { imgui::sys::igEndPopup() };
        }

        Self {
            is_open,
            delete_requested,
        }
    }
}

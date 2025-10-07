use std::ffi::CString;
use std::ptr;

use ris_asset::assets::ris_scene;
use ris_asset_data::asset_id::AssetId;
use ris_data::ecs::decl::GameObjectHandle;
use ris_data::ecs::id::GameObjectKind;
use ris_data::ecs::id::SceneKind;
use ris_data::god_state::GodState;
use ris_error::RisResult;

use crate::inspector_util;
use crate::ui_helper::selection::Selection;
use crate::ui_helper::IUiHelperModule;
use crate::ui_helper::SharedStateWeakPtr;
use crate::ui_helper::UiHelperDrawData;

const PAYLOAD_ID: &str = "hierarchy drag drop payload id";

pub struct HierarchyModule {
    shared_state: SharedStateWeakPtr,
    selected_chunk: usize,
}

impl IUiHelperModule for HierarchyModule {
    fn name() -> &'static str {
        "hierarchy"
    }

    fn build(shared_state: SharedStateWeakPtr) -> Box<dyn IUiHelperModule> {
        Box::new(Self {
            shared_state,
            selected_chunk: 0,
        })
    }

    fn draw(&mut self, data: &mut UiHelperDrawData) -> RisResult<()> {
        let UiHelperDrawData {
            ui,
            state: GodState { scene, .. },
            ..
        } = data;

        let mut choices = Vec::with_capacity(scene.static_chunks.len() + 1);
        choices.push("dynamics".to_string());

        for i in 0..(choices.capacity() - 1) {
            let loaded = match self.shared_state.borrow_mut().chunk(i) {
                Some(AssetId::Index(id)) => id.to_string(),
                Some(AssetId::Path(id)) => id.clone(),
                None => "".to_string(),
            };

            choices.push(format!("statics {} {}", i, loaded));
        }

        ui.combo_simple_string("chunk", &mut self.selected_chunk, &choices);

        let dynamics_are_selected = self.selected_chunk == 0;
        if !dynamics_are_selected {
            let chunk_index = self.selected_chunk - 1;

            let chunk = self.shared_state.borrow_mut().chunk(chunk_index);

            let _disabled_token = ui.begin_disabled(chunk.is_none());

            if ui.button("clear") {
                scene.clear_chunk(chunk_index);
                self.shared_state.borrow_mut().set_chunk(chunk_index, None);
            }

            ui.same_line();
            if ui.button("save") {
                if let Some(AssetId::Path(path)) = chunk.clone() {
                    ris_log::debug!("saving scene... chunk: {} path: {}", chunk_index, path,);
                    let bytes = ris_scene::serialize(scene, chunk_index)?;

                    let asset_path = self.shared_state.borrow().app_info.asset_path()?;
                    let path = asset_path.join(path);
                    let mut file = std::fs::File::create(path)?;
                    ris_io::write(&mut file, &bytes)?;
                }
            }

            if let Some(AssetId::Path(path)) = chunk {
                ui.same_line();
                ui.text(path)
            }
        }

        let (chunk, kind) = if dynamics_are_selected {
            (&scene.dynamic_game_objects, GameObjectKind::Dynamic)
        } else {
            let chunk = self.selected_chunk - 1;

            (
                &scene.static_chunks[chunk].game_objects,
                GameObjectKind::Static { chunk },
            )
        };

        let child_token = ui.child_window("hierarchy child window").begin();
        if child_token.is_some() {
            let alive = chunk.iter().filter(|x| x.borrow().is_alive).count();
            ui.label_text("game objects", format!("{}/{}", alive, chunk.len()));

            if unsafe { imgui::sys::igBeginPopupContextWindow(ptr::null(), 1) } {
                if ui.menu_item("new") {
                    GameObjectHandle::new_with_kind(scene, kind)?;
                }

                unsafe { imgui::sys::igEndPopup() }
            }

            let handles = chunk
                .iter()
                .filter(|x| {
                    let handle: GameObjectHandle = x.borrow().handle.into();
                    let parent_handle = handle.parent(scene).unwrap_or(None);
                    let is_root = parent_handle.is_none();
                    let is_alive = x.borrow().is_alive;

                    is_alive && is_root
                })
                .map(|x| x.borrow().handle)
                .collect::<Vec<_>>();

            for handle in handles {
                self.draw_node(handle.into(), data)?;
            }
        }

        Ok(())
    }
}

impl HierarchyModule {
    fn draw_node(
        &mut self,
        handle: GameObjectHandle,
        data: &mut UiHelperDrawData,
    ) -> RisResult<()> {
        let UiHelperDrawData {
            ui,
            state: GodState { scene, .. },
            ..
        } = data;

        let name = handle.name(scene)?;
        let id = CString::new(format!("{}##{:?}", name, handle))?;

        let has_children = !handle.children(scene)?.is_empty();
        let is_selected = {
            let aref = self.shared_state.borrow();
            let selected = aref.selector.get_selection();
            selected
                .map(|x| match x {
                    Selection::GameObject(x) => x.is_alive(scene) && x == handle,
                    _ => false,
                })
                .unwrap_or(false)
        };

        let mut flags = 0;

        flags |= 1 << 7; // ImGuiTreeNodeFlags_OpenOnArrow
        flags |= 1 << 6; // ImGuiTreeNodeFlags_OpenOnDoubleClick
        flags |= 1 << 11; // ImGuiTreeNodeFlags_SpanAvailWidth
        flags |= 1 << 15; // ImGuiTreeNodeFlags_NavLeftJumpsBackHere

        if is_selected {
            flags |= 1 << 0; // ImGuiTreeNodeFlags_Selected
        }
        if !has_children {
            flags |= 1 << 8; // ImGuiTreeNodeFlags_Leaf
        }

        let open = unsafe { imgui::sys::igTreeNodeEx_Str(id.as_ptr(), flags) };

        if unsafe { imgui::sys::igBeginPopupContextItem(ptr::null(), 1) } {
            {
                let _disabled_token = ui.begin_disabled(handle.parent(scene)?.is_none());
                if ui.menu_item("unparent") {
                    handle.set_parent(scene, None, usize::MAX)?;
                }
            }

            if ui.menu_item("new") {
                let kind = handle.scene_id().kind;
                let is_game_object = matches!(
                    kind,
                    SceneKind::DynamicGameObject | SceneKind::StaticGameObjct { .. }
                );

                if !is_game_object {
                    return ris_error::new_result!("handle id was not a gameobject");
                }
                let child = GameObjectHandle::new_with_kind(scene, kind.try_into()?)?;
                child.set_parent(scene, Some(handle), usize::MAX)?;
                ris_log::debug!("parent: {:?}", handle);
            }

            if ui.menu_item("destroy") {
                handle.destroy(scene);
            }

            unsafe { imgui::sys::igEndPopup() };
        }

        if unsafe { imgui::sys::igIsItemClicked(0) && !imgui::sys::igIsItemToggledOpen() } {
            let selection = Some(Selection::GameObject(handle));
            self.shared_state
                .borrow_mut()
                .selector
                .set_selection(selection);
        }

        if let Some(guard) = inspector_util::drag_drop_source() {
            let mut aref_mut = self.shared_state.borrow_mut();
            aref_mut.set_drag_drop_payload(&guard, PAYLOAD_ID, handle)?;
            ui.text(name);
        }

        if let Some(guard) = inspector_util::drag_drop_target() {
            let mut aref_mut = self.shared_state.borrow_mut();

            let payload =
                aref_mut.accept_drag_drop_payload::<GameObjectHandle>(&guard, PAYLOAD_ID)?;
            if let Some(dragged_handle) = payload {
                ris_log::info!("accepted drag");

                if let Err(e) = dragged_handle.set_parent(scene, Some(handle), 0) {
                    ris_log::error!("failed to drag: {}", e);
                }
            }
        }

        if open {
            if handle.is_alive(scene) {
                for child in handle.children(scene)? {
                    self.draw_node(child, data)?;
                }
            }

            unsafe { imgui::sys::igTreePop() };
        }

        Ok(())
    }
}

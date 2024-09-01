use std::ffi::CStr;
use std::ffi::CString;
use std::ptr;

use ris_data::ecs::decl::GameObjectHandle;
use ris_data::ecs::id::EcsObject;
use ris_data::ecs::id::SceneId;
use ris_data::ecs::id::SceneKind;
use ris_data::ecs::id::GameObjectKind;
use ris_data::god_state::GodState;
use ris_error::RisResult;

use crate::ui_helper::selection::Selection;
use crate::ui_helper::IUiHelperModule;
use crate::ui_helper::SharedStateWeakPtr;
use crate::ui_helper::UiHelperDrawData;

const PAYLOAD_ID: &CStr = c"hierarchy drag drop payload id";

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

        let mut choices = Vec::with_capacity(scene.static_game_objects.len() + 1);
        choices.push("movables".to_string());

        for i in 0..(choices.capacity() - 1) {
            choices.push(format!("statics {}", i));
        }

        ui.combo_simple_string("chunk", &mut self.selected_chunk, &choices);

        let (chunk, kind) = if self.selected_chunk == 0 {
            (&scene.movable_game_objects, GameObjectKind::Movable)
        } else {
            let chunk = self.selected_chunk - 1;

            (&scene.static_game_objects[chunk], GameObjectKind::Static { chunk })
        };

        let alive = chunk
            .iter()
            .filter(|x| x.borrow().is_alive)
            .count();
        ui.label_text("game objects", format!("{}/{}", alive, chunk.len()));

        if unsafe { imgui::sys::igBeginPopupContextWindow(ptr::null(), 1) } {
            if ui.menu_item("new") {
                GameObjectHandle::new(scene, kind)?;
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

        Ok(())
    }
}

impl HierarchyModule {
    fn draw_node(
        &mut self,
        handle: GameObjectHandle,
        data: &mut UiHelperDrawData,
    ) -> RisResult<()> {
        ris_debug::gizmo::view_point(
            handle.world_position(&data.state.scene)?,
            handle.world_rotation(&data.state.scene)?,
            None,
        )?;

        let UiHelperDrawData {
            ui,
            state: GodState { scene, .. },
            ..
        } = data;

        let name = handle.name(scene)?;
        let id = CString::new(format!("{}##{:?}", name, handle))?;

        let has_children = handle.child_len(scene)? > 0;
        let is_selected = {
            let aref = self.shared_state.borrow();
            let selected = aref.selector.get_selection();
            selected
                .map(|x| match x {
                    Selection::GameObject(x) => x.is_alive(scene) && x == handle,
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
            if ui.menu_item("new") {
                let kind = handle.scene_id().kind;
                let is_game_object = match kind {
                    SceneKind::Null => false,
                    SceneKind::MovableGameObject => true,
                    SceneKind::StaticGameObjct { .. } => true,
                    SceneKind::Component => false,
                };

                if !is_game_object {
                    return ris_error::new_result!("handle id was not a gameobject");
                }
                let child = GameObjectHandle::new(scene, kind.try_into()?)?;
                child.set_parent(scene, Some(handle), usize::MAX, false)?;
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

        if unsafe { imgui::sys::igBeginDragDropSource(0) } {
            unsafe {
                let payload = Box::new(handle);
                let payload_ptr = Box::leak(payload);

                // wtf rust
                // is there really no easier way to cast to *void?
                let data = payload_ptr as *const GameObjectHandle as *const std::ffi::c_void;

                imgui::sys::igSetDragDropPayload(
                    PAYLOAD_ID.as_ptr(),
                    data,
                    std::mem::size_of::<GameObjectHandle>(),
                    0,
                );

                let drag_text = CString::new(name)?;
                imgui::sys::igText(drag_text.as_ptr());

                imgui::sys::igEndDragDropSource();
            }
        }

        if unsafe { imgui::sys::igBeginDragDropTarget() } {
            unsafe {
                let payload = imgui::sys::igAcceptDragDropPayload(PAYLOAD_ID.as_ptr(), 0);
                if !payload.is_null() {
                    let data_ptr = (*payload).Data as *const GameObjectHandle;
                    let dragged_handle = *data_ptr;

                    if let Err(e) = dragged_handle.set_parent(scene, Some(handle), 0, true) {
                        ris_log::error!("failed to drag: {}", e);
                    }
                }

                imgui::sys::igEndDragDropTarget();
            }
        }

        if open {
            if handle.is_alive(scene) {
                let children = handle.child_iter(scene)?.collect::<Vec<_>>();
                for child in children {
                    self.draw_node(child, data)?;
                }
            }

            unsafe { imgui::sys::igTreePop() };
        }

        Ok(())
    }
}

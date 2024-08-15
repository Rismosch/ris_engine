use std::ffi::CStr;
use std::ffi::CString;
use std::ptr;

use ris_data::game_object::GameObjectHandle;
use ris_data::game_object::GameObjectId;
use ris_data::game_object::GameObjectKind;
use ris_data::god_state::GodState;
use ris_error::RisResult;

use super::IUiHelperModule;
use super::UiHelperDrawData;

const PAYLOAD_ID: &CStr = c"hierarchy drag drop payload id";

pub struct HierarchyModule {
    selected_chunk: usize,
    selected_game_object: Option<GameObjectId>,
}

impl IUiHelperModule for HierarchyModule {
    fn name() -> &'static str {
        "hierarchy"
    }

    fn build(_app_info: &ris_data::info::app_info::AppInfo) -> Box<dyn IUiHelperModule> {
        Box::new(Self{
            selected_chunk: 0,
            selected_game_object: None,
        })
    }

    fn draw(&mut self, data: &mut UiHelperDrawData) -> RisResult<()> {
        let UiHelperDrawData {
            ui,
            state: GodState { scene, .. },
            ..
        } = data;

        let mut choices = Vec::with_capacity(scene.statics.len() + 1);
        choices.push("movables".to_string());
        
        for i in 0..(choices.capacity() - 1) {
            choices.push(format!("statics {}", i));
        }

        ui.combo_simple_string("chunk", &mut self.selected_chunk, &choices);

        let (chunk, kind) = if self.selected_chunk == 0 {
            (
                &scene.movables,
                GameObjectKind::Movable,
            )
        } else {
            let chunk = self.selected_chunk - 1;
            
            (
                &scene.statics[chunk],
                GameObjectKind::Static{chunk},
            )
        };

        let available = scene.count_available_game_objects(kind);

        ui.label_text("available", format!("{}/{}", available, chunk.len()));

        if unsafe {imgui::sys::igBeginPopupContextWindow(ptr::null(), 1)} {
            if ui.menu_item("new") {
                GameObjectHandle::new(&scene, kind)?;
            }

            unsafe {imgui::sys::igEndPopup()}
        }

        let handles = chunk.iter()
            .filter(|x| {
                let handle = x.borrow().handle();
                let parent_handle = handle.parent(&scene).unwrap_or(None);
                let is_root = parent_handle.is_none();
                let is_alive = x.borrow().is_alive();

                is_alive && is_root
            })
            .map(|x| x.borrow().handle())
            .collect::<Vec<_>>();

        for handle in handles {
            self.draw_node(handle, data)?;
        }

        Ok(())
    }
}

impl HierarchyModule {
    fn draw_node(&mut self, handle: GameObjectHandle, data: &mut UiHelperDrawData) -> RisResult<()> {
        let UiHelperDrawData {
            ui,
            state: GodState { scene, .. },
            ..
        } = data;

        let name = handle.name(scene)?;
        let id = CString::new(format!("{}#{:?}", name, handle))?;

        let has_children = handle.child_len(&scene)? > 0;
        let is_selected = self.selected_game_object
            .map(|x| x == handle.id).unwrap_or(false);

        let mut flags = 0;
        if is_selected {
            flags |= 1 << 0; // ImGuiTreeNodeFlags_Selected
        }
        if !has_children {
            flags |= 1 << 8; // ImGuiTreeNodeFlags_Leaf
        }

        let open = unsafe {imgui::sys::igTreeNodeEx_Str(id.as_ptr(), flags)};

        if unsafe {imgui::sys::igBeginPopupContextItem(ptr::null(), 1)}  {
            if ui.menu_item("new") {
                let child = GameObjectHandle::new(&scene, handle.id.kind)?;
                child.set_parent(&scene, Some(handle), usize::MAX)?;
                ris_log::debug!("parent: {:?}", handle);
            }

            if ui.menu_item("delete") {
                handle.destroy(&scene);
            }

            unsafe {imgui::sys::igEndPopup()};
        }

        if unsafe {imgui::sys::igIsItemClicked(0)} {
            self.selected_game_object = Some(handle.id);
        }

        if unsafe {imgui::sys::igBeginDragDropSource(0)} {
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

        if unsafe {imgui::sys::igBeginDragDropTarget()} {
            unsafe {
                let payload = imgui::sys::igAcceptDragDropPayload(PAYLOAD_ID.as_ptr(), 0);
                if !payload.is_null() {
                    let data_ptr = (*payload).Data as *const GameObjectHandle;
                    let dragged_handle = *data_ptr;

                    if let Err(e) = dragged_handle.set_parent(&scene, Some(handle), 0) {
                        ris_log::error!("failed to drag: {}", e);
                    }
                }

                imgui::sys::igEndDragDropTarget();
            }
        }

        if open {
            if handle.is_alive(&scene) {
                let children = handle.child_iter(&scene)?.collect::<Vec<_>>();
                for child in children {
                    self.draw_node(child, data)?;
                }
            }

            unsafe {imgui::sys::igTreePop()};
        }

        Ok(())
    }
}

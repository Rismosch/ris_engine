use std::ffi::CStr;
use std::ffi::CString;
use std::ptr;

use ris_data::game_object::GameObjectHandle;
use ris_data::game_object::GameObjectKind;
use ris_data::god_state::GodState;
use ris_error::RisResult;

use super::IUiHelperModule;
use super::UiHelperDrawData;

pub struct HierarchyModule {
    selected_chunk: usize,
}

impl IUiHelperModule for HierarchyModule {
    fn name() -> &'static str {
        "hierarchy"
    }

    fn build(_app_info: &ris_data::info::app_info::AppInfo) -> Box<dyn IUiHelperModule> {
        Box::new(Self{
            selected_chunk: 0,
        })
    }

    fn draw(&mut self, data: &mut UiHelperDrawData) -> RisResult<()> {
        let UiHelperDrawData {
            ui,
            state: GodState { scene, .. },
            ..
        } = data;

        //ui.show_demo_window(&mut true);

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

        let id = "game objects";
        let cstr_id = CString::new(id)?;
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
            draw_node(handle, data)?;
        }

        Ok(())
    }
}

fn draw_node(handle: GameObjectHandle, data: &mut UiHelperDrawData) -> RisResult<()> {
    let UiHelperDrawData {
        ui,
        state: GodState { scene, .. },
        ..
    } = data;

    let name = format!("game object {}", handle.id.index);
    let name_cstr = CString::new(name)?;

    let open = unsafe {imgui::sys::igTreeNode_Str(name_cstr.as_ptr())};

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

    //super::util::right_click_menu("id", || {
    //    if ui.menu_item("new") {
    //        let child = GameObjectHandle::new(&scene, handle.id.kind)?;
    //        child.set_parent(&scene, Some(handle), usize::MAX)?;
    //        ris_log::debug!("parent: {:?}", handle);
    //    }

    //    if ui.menu_item("delete") {
    //        handle.destroy(&scene);
    //    }

    //    Ok(())
    //})?;

    if open {
        if handle.is_alive(&scene) {
            let children = handle.child_iter(&scene)?.collect::<Vec<_>>();
            for child in children {
                draw_node(child, data)?;
            }
        }

        unsafe {imgui::sys::igTreePop()};
    }

    Ok(())
}

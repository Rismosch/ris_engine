use std::ffi::CString;
use std::path::Path;
use std::path::PathBuf;

use ris_asset_data::AssetId;
use ris_asset::assets::ris_scene;
use ris_data::ecs::scene::Scene;
use ris_data::ecs::scene::SceneCreateInfo;
use ris_error::Extensions;
use ris_error::RisResult;

use crate::inspector_util;
use crate::ui_helper::selection::Selection;
use crate::ui_helper::IUiHelperModule;
use crate::ui_helper::SharedStateWeakPtr;
use crate::ui_helper::UiHelperDrawData;

pub struct AssetBrowser {
    shared_state: SharedStateWeakPtr,
    clicked_path: Option<PathBuf>,
    is_dragging: bool,
}

impl IUiHelperModule for AssetBrowser {
    fn name() -> &'static str {
        "assets"
    }

    fn build(shared_state: SharedStateWeakPtr) -> Box<dyn IUiHelperModule> {
        Box::new(Self {
            shared_state,
            clicked_path: None,
            is_dragging: false,
        })
    }

    fn draw(&mut self, data: &mut UiHelperDrawData) -> RisResult<()> {
        let root = self.shared_state.borrow().app_info.asset_path()?;

        if !root.is_dir() {
            data.ui.text("not available");
            return Ok(());
        }

        let entries = get_sorted_children(&root)?;
        for entry in entries {
            self.draw_asset_recursive(&root, entry, data)?;
        }

        Ok(())
    }
}

impl AssetBrowser {
    fn draw_asset_recursive(
        &mut self,
        root: impl AsRef<Path>,
        path: impl AsRef<Path>,
        data: &mut UiHelperDrawData,
    ) -> RisResult<()> {
        let root = root.as_ref();
        let path = path.as_ref();
        let path_without_root = path.strip_prefix(root)?;

        let selection = self.shared_state.borrow().selector.get_selection();
        let is_selected = selection
            .map(|x| match x {
                Selection::AssetPath(selected_path) => selected_path == path_without_root,
                _ => false,
            })
            .unwrap_or(false);

        let file_name = path
            .file_name()
            .into_ris_error()?
            .to_str()
            .into_ris_error()?;

        let empty_path = PathBuf::from("");
        let parent_path = path
            .parent()
            .unwrap_or(&empty_path)
            .to_str()
            .into_ris_error()?;

        let id = CString::new(format!("{}##{}", file_name, parent_path))?;

        let mut flags = 0;

        if path.is_dir() {
            flags |= 1 << 7; // ImGuiTreeNodeFlags_OpenOnArrow
            flags |= 1 << 6; // ImGuiTreeNodeFlags_OpenOnDoubleClick
            flags |= 1 << 11; // ImGuiTreeNodeFlags_SpanAvailWidth
            flags |= 1 << 15; // ImGuiTreeNodeFlags_NavLeftJumpsBackHere
        } else {
            flags |= 1 << 8; // ImGuiTreeNodeFlags_Leaf
            flags |= 1 << 11; // ImGuiTreeNodeFlags_SpanAvailWidth
            flags |= 1 << 15; // ImGuiTreeNodeFlags_NavLeftJumpsBackHere
        }

        if is_selected {
            flags |= 1 << 0; // ImGuiTreeNodeFlags_Selected
        }

        let is_open = unsafe { imgui::sys::igTreeNodeEx_Str(id.as_ptr(), flags) };

        // context menu
        if path.is_dir() && unsafe { imgui::sys::igBeginPopupContextItem(std::ptr::null(), 1) } {
            if data.ui.menu_item("new scene") {
                let mut new_path =
                    PathBuf::from(path).join(format!("new.{}", ris_scene::EXTENSION));
                let mut counter = 0;
                while new_path.exists() {
                    counter += 1;
                    new_path = PathBuf::from(path).join(format!(
                        "new({}).{}",
                        counter,
                        ris_scene::EXTENSION
                    ));
                }

                let scene_create_info = SceneCreateInfo::with_single_static_chunk(data.state.scene.registry.clone());
                let empty_scene = Scene::new(scene_create_info)?;
                let scene_bytes = ris_scene::serialize(&empty_scene, 0)?;

                let mut file = std::fs::File::create_new(new_path)?;
                ris_io::write(&mut file, &scene_bytes)?;
            }

            unsafe { imgui::sys::igEndPopup() };
        } else if path.is_file()
            && unsafe { imgui::sys::igBeginPopupContextItem(std::ptr::null(), 1) }
        {
            if data.ui.menu_item("delete") {
                if let Err(e) = std::fs::remove_file(path) {
                    ris_log::error!("failed to delete file: {}", e)
                }
            }

            unsafe { imgui::sys::igEndPopup() };
        }

        // drag and drop
        if path.is_file() {
            if let Some(guard) = inspector_util::drag_drop_source() {
                let asset_id = AssetId::Path(path_without_root.display().to_string());
                let mut aref_mut = self.shared_state.borrow_mut();
                aref_mut.set_drag_drop_payload(
                    &guard,
                    "asset",
                    asset_id,
                )?;
                data.ui.text(file_name);
                self.is_dragging = true;
            }
        }

        // click
        // dragging takes several frames to be detected, but click goes through on frame 1. as
        // such, we must jump through some hoops to detect whether the item was clicked or being
        // dragged
        if unsafe {imgui::sys::igIsItemClicked(0) && !imgui::sys::igIsItemToggledOpen()} {
            self.clicked_path = Some(path_without_root.to_path_buf());
        }

        if unsafe {imgui::sys::igIsMouseReleased_Nil(0)} {
            if let Some(clicked_path) = self.clicked_path.take() {
                if !self.is_dragging {
                    let selection = Some(Selection::AssetPath(clicked_path));
                    ris_log::debug!(
                        "select: \"{:?}\" path: \"{:?}\" root: \"{:?}\"",
                        selection,
                        path,
                        root
                    );
                    self.shared_state
                        .borrow_mut()
                        .selector
                        .set_selection(selection);
                }

                self.is_dragging = false;
            }
        }

        // draw children
        if !is_open {
            return Ok(());
        }

        if !path.is_dir() {
            unsafe { imgui::sys::igTreePop() };
            return Ok(());
        }

        let entries = get_sorted_children(path)?;
        for entry_path in entries {
            self.draw_asset_recursive(root, entry_path, data)?;
        }

        unsafe { imgui::sys::igTreePop() };

        Ok(())
    }
}

fn get_sorted_children(path: impl AsRef<Path>) -> RisResult<Vec<PathBuf>> {
    let path = path.as_ref();

    let entries = std::fs::read_dir(path)?;
    let mut mapped_entries = entries
        .into_iter()
        .filter_map(|x| match x {
            Ok(dir_entry) => match dir_entry.metadata() {
                Ok(metadata) => Some((dir_entry.path(), metadata)),
                Err(_) => None,
            },
            Err(_) => None,
        })
        .collect::<Vec<_>>();

    mapped_entries.sort_by(|left, right| {
        let (left_path, left_metadata) = left;
        let (right_path, right_metadata) = right;

        if left_metadata.is_dir() && !right_metadata.is_dir() {
            std::cmp::Ordering::Less
        } else if !left_metadata.is_dir() && right_metadata.is_dir() {
            std::cmp::Ordering::Greater
        } else {
            left_path.cmp(right_path)
        }
    });

    let sorted_entries = mapped_entries
        .into_iter()
        .map(|(path, _metadata)| path)
        .collect::<Vec<_>>();

    Ok(sorted_entries)
}

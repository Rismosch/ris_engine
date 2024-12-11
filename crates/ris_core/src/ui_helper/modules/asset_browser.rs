use std::ffi::CString;
use std::path::Path;
use std::path::PathBuf;

use ris_error::Extensions;
use ris_error::RisResult;

use crate::ui_helper::IUiHelperModule;
use crate::ui_helper::SharedStateWeakPtr;
use crate::ui_helper::UiHelperDrawData;

pub struct AssetBrowser{
    shared_state: SharedStateWeakPtr,
}

impl IUiHelperModule for AssetBrowser {
    fn name() -> &'static str {
        "asset browser"
    }

    fn build(shared_state: SharedStateWeakPtr) -> Box<dyn IUiHelperModule> {
        Box::new(Self{
            shared_state,
        })
    }

    fn draw(&mut self, data: &mut UiHelperDrawData) -> RisResult<()> {
        let UiHelperDrawData {
            ui,
            ..
        } = data;

        let root = self.shared_state.borrow().app_info.asset_path()?;
        let root = Path::new(&root);

        self.draw_asset_recursive(root, data)?;

        Ok(())
    }
}

impl AssetBrowser {
    fn draw_asset_recursive(
        &mut self,
        path: impl AsRef<Path>,
        data: &mut UiHelperDrawData,
    ) -> RisResult<()> {
        let UiHelperDrawData {
            ui,
            ..
        } = data;

        let path = path.as_ref();

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

        // draw directory
        if path.is_dir() {

            let mut flags = 0;
            flags |= 1 << 7; // ImGuiTreeNodeFlags_OpenOnArrow
            flags |= 1 << 6; // ImGuiTreeNodeFlags_OpenOnDoubleClick
            flags |= 1 << 11; // ImGuiTreeNodeFlags_SpanAvailWidth
            flags |= 1 << 15; // ImGuiTreeNodeFlags_NavLeftJumpsBackHere

            let is_open = unsafe { imgui::sys::igTreeNodeEx_Str(id.as_ptr(), flags) };

            if !is_open {
                return Ok(())
            }

            let entries = std::fs::read_dir(path)?;
            let mut sorted_entries = entries
                .into_iter()
                .filter_map(|x| {
                    match x {
                        Ok(dir_entry) => match dir_entry.metadata() {
                            Ok(metadata) => Some((dir_entry.path(), metadata)),
                            Err(_) => None,
                        },
                        Err(_) => None,
                    }
                })
                .collect::<Vec<_>>();
            sorted_entries.sort_by(|left, right| {
                let (left_path, left_metadata) = left;
                let (right_path, right_metadata) = right;

                if left_metadata.is_dir() && !right_metadata.is_dir() {
                    std::cmp::Ordering::Less
                } else if !left_metadata.is_dir() && right_metadata.is_dir() {
                    std::cmp::Ordering::Greater
                } else {
                    left_path.cmp(&right_path)
                }
            });

            for (entry_path, _metadata) in sorted_entries {
                self.draw_asset_recursive(entry_path, data)?;
            }

            unsafe {imgui::sys::igTreePop()};

            return Ok(())
        } // draw directory end

        ui.text(file_name);
        //let mut flags = 0;
        ////flags |= 1 << 9; // ImGuiTreeNodeFlags_Bullet

        //let is_open = unsafe {imgui::sys::igTreeNodeEx_Str(id.as_ptr(), flags)};
        //if is_open {
        //    unsafe {imgui::sys::igTreePop()};
        //}

        Ok(())
    }
}

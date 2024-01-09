use ris_asset::loader::scenes_loader::Scenes;
use ris_error::RisResult;

use crate::vulkan::shader;

pub struct ImguiRenderer {

}

impl ImguiRenderer {
    #[cfg(debug_assertions)]
    pub fn init(scenes: Scenes) -> RisResult<Option<Self>> {
        // shaders
        //let vs_future = shader::load_async(scenes.imgui_vs.clone());
        //let fs_future = asset_loader::load(scenes.imgui_vs.clone());

        Ok(Some(Self{}))
    }

    #[cfg(not(debug_assertions))]
    pub fn init() -> RisResult<Option<Self>> {
        Ok(None)
    }
}

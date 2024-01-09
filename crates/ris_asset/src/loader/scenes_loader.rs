use ris_error::RisResult;

use crate::loader::ris_loader;
use crate::AssetId;

#[derive(Clone)]
pub struct Scenes {
    pub default_vs: AssetId,
    pub default_fs: AssetId,
    pub imgui_vs: AssetId,
    pub imgui_fs: AssetId,
}

pub fn load(bytes: &[u8]) -> RisResult<Scenes> {
    let data = ris_error::unroll_option!(
        ris_loader::load(bytes)?,
        "failed to load ris asset from scenes"
    )?;

    let default_vs = data.references[0].clone();
    let default_fs = data.references[1].clone();
    let imgui_vs = data.references[2].clone();
    let imgui_fs = data.references[3].clone();

    let scenes = Scenes {
        default_vs,
        default_fs,
        imgui_vs,
        imgui_fs,
    };

    Ok(scenes)
}


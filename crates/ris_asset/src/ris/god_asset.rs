use ris_error::Extensions;
use ris_error::RisResult;

use crate::ris::Header;
use crate::AssetId;

#[derive(Clone)]
pub struct GodAsset {
    pub default_vs: AssetId,
    pub default_fs: AssetId,
    pub imgui_vs: AssetId,
    pub imgui_fs: AssetId,
}

impl GodAsset{
    pub fn load(bytes: &[u8]) -> RisResult<Self> {
        let data = Header::load(bytes)?.unroll()?;

        let default_vs = data.references[0].clone();
        let default_fs = data.references[1].clone();
        let imgui_vs = data.references[2].clone();
        let imgui_fs = data.references[3].clone();

        let god_asset = Self {
            default_vs,
            default_fs,
            imgui_vs,
            imgui_fs,
        };

        Ok(god_asset)
    }
}

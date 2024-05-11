use ris_error::Extensions;
use ris_error::RisResult;

use crate::RisHeader;
use crate::AssetId;

#[derive(Clone)]
pub struct RisGodAsset {
    pub default_vs: AssetId,
    pub default_fs: AssetId,
    pub imgui_vs: AssetId,
    pub imgui_fs: AssetId,
}

impl RisGodAsset{
    pub fn load(bytes: &[u8]) -> RisResult<Self> {
        let header = RisHeader::load(bytes)?.unroll()?;

        let default_vs = header.references[0].clone();
        let default_fs = header.references[1].clone();
        let imgui_vs = header.references[2].clone();
        let imgui_fs = header.references[3].clone();

        let mut cursor = std::io::Cursor::new(bytes);
        let data = ris_file::io::read_unsized(&mut cursor, header.p_content)?;
        let data_message = String::from_utf8(data)?;
        ris_log::debug!("god asset content: {}", data_message);

        let god_asset = Self {
            default_vs,
            default_fs,
            imgui_vs,
            imgui_fs,
        };

        Ok(god_asset)
    }
}

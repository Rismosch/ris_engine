use ris_error::Extensions;
use ris_error::RisResult;

use crate::RisHeader;
use crate::AssetId;

#[derive(Clone)]
pub struct RisGodAsset {
    pub default_vert_spv: AssetId,
    pub default_frag_spv: AssetId,
    pub imgui_vert_spv: AssetId,
    pub imgui_frag_spv: AssetId,
    pub texture: AssetId,
}

impl RisGodAsset{
    pub fn load(bytes: &[u8]) -> RisResult<Self> {
        let header = RisHeader::load(bytes)?.unroll()?;

        let default_vert_spv = header.references[0].clone();
        let default_frag_spv = header.references[1].clone();
        let imgui_vert_spv = header.references[2].clone();
        let imgui_frag_spv = header.references[3].clone();
        let texture = header.references[4].clone();

        let mut cursor = std::io::Cursor::new(bytes);
        let data = ris_file::io::read_unsized(&mut cursor, header.p_content)?;
        let data_message = String::from_utf8(data)?;
        ris_log::debug!("god asset content: {}", data_message);

        let god_asset = Self {
            default_vert_spv,
            default_frag_spv,
            imgui_vert_spv,
            imgui_frag_spv,
            texture,
        };

        Ok(god_asset)
    }
}

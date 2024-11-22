use ris_error::Extensions;
use ris_error::RisResult;

use crate::AssetId;
use crate::RisHeader;

#[derive(Clone)]
pub struct RisGodAsset {
    pub default_vert_spv: AssetId,
    pub default_frag_spv: AssetId,
    pub imgui_vert_spv: AssetId,
    pub imgui_frag_spv: AssetId,
    pub gizmo_segment_vert_spv: AssetId,
    pub gizmo_segment_geom_spv: AssetId,
    pub gizmo_segment_frag_spv: AssetId,
    pub gizmo_text_vert_spv: AssetId,
    pub gizmo_text_geom_spv: AssetId,
    pub gizmo_text_frag_spv: AssetId,
    pub debug_font_texture: AssetId,
    pub texture: AssetId,
}

impl RisGodAsset {
    pub fn load(bytes: &[u8]) -> RisResult<Self> {
        println!("hoi");
        let header = RisHeader::load(bytes)?.unroll()?;
        println!("poi");

        let default_vert_spv = header.references[0].clone();
        let default_frag_spv = header.references[1].clone();
        let imgui_vert_spv = header.references[2].clone();
        let imgui_frag_spv = header.references[3].clone();
        let gizmo_segment_vert_spv = header.references[4].clone();
        let gizmo_segment_geom_spv = header.references[5].clone();
        let gizmo_segment_frag_spv = header.references[6].clone();
        let gizmo_text_vert_spv = header.references[7].clone();
        let gizmo_text_geom_spv = header.references[8].clone();
        let gizmo_text_frag_spv = header.references[9].clone();
        let debug_font_texture = header.references[10].clone();
        let texture = header.references[11].clone();

        let mut cursor = std::io::Cursor::new(bytes);
        let data = ris_file::io::read_unsized(&mut cursor, header.p_content)?;
        let data_message = String::from_utf8(data)?;
        ris_log::debug!("god asset content: {}", data_message);

        let god_asset = Self {
            default_vert_spv,
            default_frag_spv,
            imgui_vert_spv,
            imgui_frag_spv,
            gizmo_segment_vert_spv,
            gizmo_segment_geom_spv,
            gizmo_segment_frag_spv,
            gizmo_text_vert_spv,
            gizmo_text_geom_spv,
            gizmo_text_frag_spv,
            debug_font_texture,
            texture,
        };

        Ok(god_asset)
    }
}

use std::io::Cursor;

use ris_error::Extensions;
use ris_error::RisResult;
use ris_io::FatPtr;

use crate::AssetId;
use crate::RisHeader;

// ris_god_asset\0\0\0
pub const MAGIC: [u8; 16] = [0x72,0x69,0x73,0x5f,0x67,0x6f,0x64,0x5f,0x61,0x73,0x73,0x65,0x74,0x00,0x00,0x00]; 
pub const PATH: &str = "god_asset.ris_god_asset";
pub const UNNAMED_PATH: &str = "asset_0";

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
    pub fn serialize(&self) -> RisResult<Vec<u8>> {
        let header = RisHeader::new(
            MAGIC,
            vec![
                self.default_vert_spv.clone(),
                self.default_frag_spv.clone(),
                self.imgui_vert_spv.clone(),
                self.imgui_frag_spv.clone(),
                self.gizmo_segment_vert_spv.clone(),
                self.gizmo_segment_geom_spv.clone(),
                self.gizmo_segment_frag_spv.clone(),
                self.gizmo_text_vert_spv.clone(),
                self.gizmo_text_geom_spv.clone(),
                self.gizmo_text_frag_spv.clone(),
                self.debug_font_texture.clone(),
                self.texture.clone(),
            ],
        );
        let header_bytes = header.serialize()?;

        let mut stream = Cursor::new(Vec::new());
        ris_io::write(&mut stream, &header_bytes)?;
        let bytes = stream.into_inner();

        Ok(bytes)
    }

    pub fn load(bytes: &[u8]) -> RisResult<Self> {
        let header = RisHeader::load(bytes)?.into_ris_error()?;
        header.assert_magic(MAGIC)?;

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

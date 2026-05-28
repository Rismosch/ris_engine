use std::mem::MaybeUninit;

use ris_asset_data::AssetId;
use ris_error::prelude::*;

use crate::assets::ris_asset::RisAsset;
use crate::codecs::json::JsonObject;

pub const PATH: &str = "god_asset.ris_god_asset";
pub const UNNAMED_PATH: &str = "asset_0";

pub const DEFAULT_VERT_SPV: &str = "default_vert_spv";
pub const DEFAULT_FRAG_SPV: &str = "default_frag_spv";
pub const TERRAIN_VERT_SPV: &str = "terrain_vert_spv";
pub const TERRAIN_FRAG_SPV: &str = "terrain_frag_spv";
pub const IMGUI_VERT_SPV: &str = "imgui_vert_spv";
pub const IMGUI_FRAG_SPV: &str = "imgui_frag_spv";
pub const GIZMO_SEGMENT_VERT_SPV: &str = "gizmo_segment_vert_spv";
pub const GIZMO_SEGMENT_FRAG_SPV: &str = "gizmo_segment_frag_spv";
pub const GIZMO_TEXT_VERT_SPV: &str = "gizmo_text_vert_spv";
pub const GIZMO_TEXT_GEOM_SPV: &str = "gizmo_text_geom_spv";
pub const GIZMO_TEXT_FRAG_SPV: &str = "gizmo_text_frag_spv";
pub const DEBUG_FONT_TEXTURE: &str = "debug_font_texture";
pub const TEXTURE: &str = "texture";

#[derive(Clone)]
pub struct RisGodAsset {
    pub default_vert_spv: AssetId,
    pub default_frag_spv: AssetId,
    pub terrain_vert_spv: AssetId,
    pub terrain_frag_spv: AssetId,
    pub imgui_vert_spv: AssetId,
    pub imgui_frag_spv: AssetId,
    pub gizmo_segment_vert_spv: AssetId,
    pub gizmo_segment_frag_spv: AssetId,
    pub gizmo_text_vert_spv: AssetId,
    pub gizmo_text_geom_spv: AssetId,
    pub gizmo_text_frag_spv: AssetId,
    pub debug_font_texture: AssetId,
    pub texture: AssetId,
}

fn asset_id_from_json(json: &JsonObject, name: impl AsRef<str>) -> RisResult<AssetId> {
    let name = name.as_ref();
    let value = json.get::<&str>(name)
        .ris_expect(&format!("id \"{}\" to be assigned", name))?;
    Ok(AssetId::from_path(value))
}

impl RisAsset for RisGodAsset {
    fn from_json(s: &mut MaybeUninit<Self>, json: &JsonObject) -> RisResult<()> {
        let s = unsafe {&mut *s.as_mut_ptr()};

        s.default_vert_spv = asset_id_from_json(json, DEFAULT_VERT_SPV)?;
        s.default_frag_spv = asset_id_from_json(json, DEFAULT_FRAG_SPV)?;
        s.terrain_vert_spv = asset_id_from_json(json, TERRAIN_VERT_SPV)?;
        s.terrain_frag_spv = asset_id_from_json(json, TERRAIN_FRAG_SPV)?;
        s.imgui_vert_spv = asset_id_from_json(json, IMGUI_VERT_SPV)?;
        s.imgui_frag_spv = asset_id_from_json(json, IMGUI_FRAG_SPV)?;
        s.gizmo_segment_vert_spv = asset_id_from_json(json, GIZMO_SEGMENT_VERT_SPV)?;
        s.gizmo_segment_frag_spv = asset_id_from_json(json, GIZMO_SEGMENT_FRAG_SPV)?;
        s.gizmo_text_vert_spv = asset_id_from_json(json, GIZMO_TEXT_VERT_SPV)?;
        s.gizmo_text_geom_spv = asset_id_from_json(json, GIZMO_TEXT_GEOM_SPV)?;
        s.gizmo_text_frag_spv = asset_id_from_json(json, GIZMO_TEXT_FRAG_SPV)?;
        s.debug_font_texture = asset_id_from_json(json, DEBUG_FONT_TEXTURE)?;
        s.texture = asset_id_from_json(json, TEXTURE)?;

        Ok(())
    }

    fn to_json(&self) -> RisResult<JsonObject> {
        let mut json = JsonObject::default();
        json.push(DEFAULT_VERT_SPV, self.default_vert_spv.path_string()?);
        json.push(DEFAULT_FRAG_SPV, self.default_frag_spv.path_string()?);
        json.push(TERRAIN_VERT_SPV, self.terrain_vert_spv.path_string()?);
        json.push(TERRAIN_FRAG_SPV, self.terrain_frag_spv.path_string()?);
        json.push(IMGUI_VERT_SPV, self.imgui_vert_spv.path_string()?);
        json.push(IMGUI_FRAG_SPV, self.imgui_frag_spv.path_string()?);
        json.push(GIZMO_SEGMENT_VERT_SPV, self.gizmo_segment_vert_spv.path_string()?);
        json.push(GIZMO_SEGMENT_FRAG_SPV, self.gizmo_segment_frag_spv.path_string()?);
        json.push(GIZMO_TEXT_VERT_SPV, self.gizmo_text_vert_spv.path_string()?);
        json.push(GIZMO_TEXT_GEOM_SPV, self.gizmo_text_geom_spv.path_string()?);
        json.push(GIZMO_TEXT_FRAG_SPV, self.gizmo_text_frag_spv.path_string()?);
        json.push(DEBUG_FONT_TEXTURE, self.debug_font_texture.path_string()?);
        json.push(TEXTURE, self.texture.path_string()?);

        Ok(json)
    }

    fn all_references_mut(&mut self) -> Vec<&mut AssetId> {
        vec![
            &mut self.default_vert_spv,
            &mut self.default_frag_spv,
            &mut self.terrain_vert_spv,
            &mut self.terrain_frag_spv,
            &mut self.imgui_vert_spv,
            &mut self.imgui_frag_spv,
            &mut self.gizmo_segment_vert_spv,
            &mut self.gizmo_segment_frag_spv,
            &mut self.gizmo_text_vert_spv,
            &mut self.gizmo_text_geom_spv,
            &mut self.gizmo_text_frag_spv,
            &mut self.debug_font_texture,
            &mut self.texture,
        ]
    }
}


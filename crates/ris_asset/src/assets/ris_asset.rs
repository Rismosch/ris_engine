use std::mem::MaybeUninit;

use ris_asset_data::AssetId;

use crate::codecs::json::JsonObject;

pub trait RisAsset : Default + Clone + Copy + Send + Sync {
    fn from_json(s: &mut MaybeUninit<Self>, json: &JsonObject);
    fn to_json(&self) -> JsonObject;
    fn all_references_mut(&mut self) -> Vec<&mut AssetId>;
}

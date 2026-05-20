use ris_asset_data::AssetId;

use crate::codecs::json::JsonObject;

pub trait RisAsset : Clone + Copy + Send + Sync {
    fn from_json(yaml: &JsonObject) -> Self;
    fn to_json(&self) -> JsonObject;
    fn all_references_mut(&mut self) -> Vec<&mut AssetId>;
}

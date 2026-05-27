use std::mem::MaybeUninit;

use ris_asset_data::AssetId;
use ris_error::prelude::*;

use crate::codecs::json::JsonObject;

pub trait RisAsset : Clone + Send {
    fn from_json(s: &mut MaybeUninit<Self>, json: &JsonObject) -> RisResult<()>;
    fn to_json(&self) -> RisResult<JsonObject>;
    fn all_references_mut(&mut self) -> Vec<&mut AssetId>;
}

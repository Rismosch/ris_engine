use std::path::Path;

use crate::asset_loader::LoadError;
use crate::asset_loader::Response;

pub struct AssetLoaderCompiled {}

impl AssetLoaderCompiled {
    pub fn new(_asset_path: &Path) -> Self {
        Self {}
    }

    pub fn load(&self, _id: u32) -> Response {
        Err(LoadError::AssetNotFound)
    }
}

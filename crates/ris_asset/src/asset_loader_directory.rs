use std::path::Path;

use crate::asset_loader::LoadError;
use crate::asset_loader::Response;

pub struct AssetLoaderDirectory {}

impl AssetLoaderDirectory {
    pub fn new(_asset_path: &Path) -> Self {
        Self {}
    }

    pub fn load(&self, _id: String) -> Response {
        Err(LoadError::AssetNotFound)
    }
}

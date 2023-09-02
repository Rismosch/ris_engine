use std::path::Path;
use std::fs::File;

use ris_util::ris_error::RisError;

use crate::asset_loader::LoadError;
use crate::asset_loader::Response;

pub struct AssetLoaderCompiled {
    file: File,
}

impl AssetLoaderCompiled {
    pub fn new(asset_path: &Path) -> Result<Self, RisError> {
        let file = ris_util::unroll!(File::open(asset_path), "failed to open \"{:?}\"", asset_path)?;

        ris_log::debug!("reading lookup");

        Ok(Self {
            file,
        })
    }

    pub fn load(&self, _id: u32) -> Response {
        Err(LoadError::AssetNotFound)
    }
}

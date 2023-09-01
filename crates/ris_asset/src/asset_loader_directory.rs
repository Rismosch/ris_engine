use std::path::Path;

use ris_util::ris_error::RisError;

pub struct AssetLoaderDirectory{}

impl AssetLoaderDirectory{
    pub fn new(asset_path: &Path) -> Self {
        Self{}
    }

    pub fn load(id: String) -> Result<Box<[u8]>, RisError> {
        ris_util::result_err!("not implemented")
    }
}

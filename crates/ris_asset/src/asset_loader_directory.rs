use std::fs::File;
use std::path::Path;
use std::path::PathBuf;

use crate::asset_loader::LoadError;
use crate::asset_loader::Response;

pub struct AssetLoaderDirectory {
    base_path: PathBuf,
}

impl AssetLoaderDirectory {
    pub fn new(asset_path: &Path) -> Self {
        let base_path = asset_path.to_path_buf();
        Self {
            base_path,
        }
    }

    pub fn load(&self, id: String) -> Response {
        let mut path = PathBuf::new();
        path.push(&self.base_path);
        path.push(id);

        let mut file = File::open(path).map_err(|_| LoadError::FileReadFailed)?;
        let file_size = 
            put seek, read and write in util function
            combine engine new and run to single function
        
        Err(LoadError::AssetNotFound)
    }
}

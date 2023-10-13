use std::fs::File;
use std::io::SeekFrom;
use std::path::Path;
use std::path::PathBuf;

use ris_util::error::RisError;

pub struct AssetLoaderDirectory {
    base_path: PathBuf,
}

impl AssetLoaderDirectory {
    pub fn new(asset_path: &Path) -> Self {
        let base_path = asset_path.to_path_buf();
        Self { base_path }
    }

    pub fn load(&self, id: String) -> Result<Vec<u8>, RisError> {
        let mut path = PathBuf::new();
        path.push(&self.base_path);
        path.push(id);

        let mut file = ris_util::unroll!(File::open(&path), "failed to open file \"{:?}\"", &path)?;
        let file_size = crate::util::seek(&mut file, SeekFrom::End(0))? as usize;
        let mut file_content = vec![0; file_size];
        crate::util::seek(&mut file, SeekFrom::Start(0))?;
        crate::util::read(&mut file, &mut file_content)?;

        Ok(file_content)
    }
}

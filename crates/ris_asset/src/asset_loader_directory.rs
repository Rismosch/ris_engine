use std::fs::File;
use std::path::Path;
use std::path::PathBuf;

use ris_error::RisResult;

pub struct AssetLoaderDirectory {
    base_path: PathBuf,
}

impl AssetLoaderDirectory {
    pub fn new(asset_path: &Path) -> Self {
        let base_path = asset_path.to_path_buf();
        Self { base_path }
    }

    pub fn load(&self, id: String) -> RisResult<Vec<u8>> {
        let mut path = PathBuf::new();
        path.push(&self.base_path);
        path.push(id);

        let mut file =
            ris_error::unroll!(File::open(&path), "failed to open file \"{:?}\"", &path,)?;
        let file_size = ris_file::seek!(&mut file, SeekFrom::End(0))? as usize;
        let mut file_content = vec![0; file_size];
        ris_file::seek!(&mut file, SeekFrom::Start(0))?;
        ris_file::read!(&mut file, file_content)?;

        Ok(file_content)
    }
}

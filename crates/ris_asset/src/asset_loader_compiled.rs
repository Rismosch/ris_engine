use std::fs::File;
use std::io::SeekFrom;
use std::path::Path;

use ris_file::io::FatPtr;
use ris_error::RisResult;

pub struct AssetLoaderCompiled {
    file: File,
    lookup: Vec<FatPtr>,
}

impl AssetLoaderCompiled {
    pub fn new(asset_path: &Path) -> RisResult<Self> {
        let mut file = File::open(asset_path)?;
        let f = &mut file;

        ris_file::io::seek(f, SeekFrom::Start(0))?;

        let mut magic_bytes = [0u8; 16];
        ris_file::io::read(f, &mut magic_bytes)?;

        if !ris_util::testing::bytes_eq(&magic_bytes, &crate::asset_compiler::MAGIC) {
            return ris_error::new_result!("unkown magic value: {:?}", magic_bytes);
        }

        let p_original_asset_names = ris_file::io::read_fat_ptr(f)?;

        let asset_lookup = ris_file::io::read_array::<crate::asset_compiler::AssetAddr>(f)?;
        let mut fat_ptr_lookup = Vec::with_capacity(asset_lookup.len());

        for i in 0..asset_lookup.len() {
            let begin = asset_lookup[i].0;
            let end = if i == asset_lookup.len() - 1 {
                p_original_asset_names.addr
            } else {
                asset_lookup[i + 1].0
            };

            let fat_ptr = FatPtr::begin_end(begin, end)?;
            fat_ptr_lookup.push(fat_ptr);
        }

        Ok(Self{
            file,
            lookup: fat_ptr_lookup,
        })
    }

    pub fn load(&mut self, id: usize) -> RisResult<Vec<u8>> {
        let p_asset = self
            .lookup
            .get(id)
            .ok_or_else(|| ris_error::new!("asset does not exist"))?;

        ris_file::io::read_unsized(&mut self.file, *p_asset)
    }
}

use std::fs::File;
use std::path::Path;

use ris_util::error::RisError;

struct AssetEntry {
    addr: u64,
    len: usize,
}

pub struct AssetLoaderCompiled {
    file: File,
    lookup: Vec<AssetEntry>,
}

impl AssetLoaderCompiled {
    pub fn new(asset_path: &Path) -> Result<Self, RisError> {
        let mut file =
            ris_util::unroll!(File::open(asset_path), "could not open compiled asset file")?;
        let f = &mut file;

        let file_size = ris_util::seek!(f, SeekFrom::End(0))?;
        ris_util::seek!(f, SeekFrom::Start(0))?;

        let mut magic_bytes = [0u8; 16];
        ris_util::read!(f, magic_bytes)?;

        if !ris_util::io::bytes_equal(&magic_bytes, &crate::asset_compiler::MAGIC) {
            return ris_util::result_err!("unkown magic value: {:?}", magic_bytes);
        }

        let mut addr_original_paths_bytes = [0u8; 8];
        ris_util::read!(f, addr_original_paths_bytes)?;
        let addr_original_paths = u64::from_le_bytes(addr_original_paths_bytes);

        let mut lookup_len_bytes = [0u8; 8];
        ris_util::read!(f, lookup_len_bytes)?;
        let lookup_len = u64::from_le_bytes(lookup_len_bytes);

        let mut lookup = Vec::with_capacity(lookup_len as usize);

        let mut next_addr_bytes = [0u8; 8];
        ris_util::read!(f, next_addr_bytes)?;
        let mut next_addr = u64::from_le_bytes(next_addr_bytes);
        for i in 0..lookup_len {
            let addr = next_addr;
            next_addr = if i == lookup_len - 1 {
                addr_original_paths
            } else {
                let mut next_addr_bytes = [0u8; 8];
                ris_util::read!(f, next_addr_bytes)?;
                u64::from_le_bytes(next_addr_bytes)
            };

            if next_addr > file_size {
                return ris_util::result_err!("asset was larger than file size");
            }

            if addr > next_addr {
                return ris_util::result_err!(
                    "current addr {} was larger than next addr {}",
                    addr,
                    next_addr
                );
            }

            let len = (next_addr - addr) as usize;
            ris_log::trace!("asset {} {} {}", i, addr, len);
            let asset_entry = AssetEntry { addr, len };

            lookup.push(asset_entry);
        }

        Ok(Self { file, lookup })
    }

    pub fn load(&mut self, id: usize) -> Result<Vec<u8>, RisError> {
        let entry = self
            .lookup
            .get(id)
            .ok_or(ris_util::new_err!("asset does not exist"))?;
        let f = &mut self.file;
        let mut bytes = vec![0u8; entry.len];
        ris_util::seek!(f, SeekFrom::Start(entry.addr))?;
        ris_util::read!(f, bytes)?;
        Ok(bytes)
    }
}

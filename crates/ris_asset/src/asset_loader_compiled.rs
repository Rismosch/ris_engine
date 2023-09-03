use std::path::Path;
use std::fs::File;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;

use crate::asset_loader::LoadError;
use crate::asset_loader::Response;

struct AssetEntry {
    addr: u64,
    len: u64,
}

pub struct AssetLoaderCompiled {
    file: File,
}

impl AssetLoaderCompiled {
    pub fn new(asset_path: &Path) -> Result<Self, LoadError> {
        let file = File::open(asset_path).map_err(|_| LoadError::FileReadFailed)?;

        let mut loader = Self{file};

        let file_size = loader.seek(SeekFrom::End(0))?;
        loader.seek(SeekFrom::Start(0))?;

        let mut version_bytes = [0u8; 16];
        loader.read(&mut version_bytes)?;

        // TODO compare version

        let mut addr_original_paths_bytes = [0u8; 8];
        loader.read(&mut addr_original_paths_bytes)?;
        let addr_original_paths = u64::from_le_bytes(addr_original_paths_bytes);

        let mut lookup_len_bytes = [0u8; 8];
        loader.read(&mut lookup_len_bytes)?;
        let lookup_len = u64::from_le_bytes(lookup_len_bytes);

        if lookup_len <= 1 {
            // TODO lookup has only 1 or no elements
            // requires special handling as loop below only works for the general case
        }


        let mut next_addr_bytes = [0u8; 8];
        loader.read(&mut next_addr_bytes)?;
        let next_addr = u64::from_le_bytes(next_addr_bytes);
        for i in 0..lookup_len {
            let addr = next_addr;
            let next_addr = if i == lookup_len - 1 {
                addr_original_paths
            } else {
                let mut next_addr_bytes = [0u8; 8];
                loader.read(&mut next_addr_bytes)?;
                u64::from_le_bytes(next_addr_bytes)
            };

            if next_addr > file_size {
                ris_log::error!("asset is supposedly bigger than file size. this is impossible, therefore asset size is incorrect");
                return Err(LoadError::FileReadFailed);

            }

            if addr > next_addr {
                ris_log::error!("current addr is larger than next addr. this should not be possible. assets may be corrupted");
                return Err(LoadError::FileReadFailed);
            }

            let len = next_addr - addr;
            ris_log::trace!("asset {} {} {}", i, addr, len);
            let asset_entry = AssetEntry{
                addr,
                len,
            };
            
        }

        Ok(loader)
    }

    pub fn load(&mut self, _id: u32) -> Response {
        Err(LoadError::AssetNotFound)
    }

    fn seek(&mut self, pos: SeekFrom) -> Result<u64, LoadError> {
        self.file.seek(pos).map_err(|_| LoadError::FileReadFailed)
    }

    fn read(&mut self, buf: &mut [u8]) -> Result<usize, LoadError> {
        let read_bytes = self.file.read(buf).map_err(|_| LoadError::FileReadFailed)?;
        if read_bytes != buf.len() {
            ris_log::error!("expected to read {} bytes but actually read {}", buf.len(), read_bytes);
            Err(LoadError::FileReadFailed)
        } else {
            Ok(read_bytes)
        }
    }
}

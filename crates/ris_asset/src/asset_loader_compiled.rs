use std::fs::File;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::path::Path;

use crate::asset_loader::LoadError;
use crate::asset_loader::Response;

struct AssetEntry {
    addr: u64,
    len: usize,
}

pub struct AssetLoaderCompiled {
    file: File,
    lookup: Vec<AssetEntry>,
}

impl AssetLoaderCompiled {
    pub fn new(asset_path: &Path) -> Result<Self, LoadError> {
        let mut file = File::open(asset_path).map_err(|_| LoadError::FileReadFailed)?;
        let f = &mut file;

        let file_size = seek(f, SeekFrom::End(0))?;
        seek(f, SeekFrom::Start(0))?;

        let mut version_bytes = [0u8; 16];
        read(f, &mut version_bytes)?;

        // TODO compare version

        let mut addr_original_paths_bytes = [0u8; 8];
        read(f, &mut addr_original_paths_bytes)?;
        let addr_original_paths = u64::from_le_bytes(addr_original_paths_bytes);

        let mut lookup_len_bytes = [0u8; 8];
        read(f, &mut lookup_len_bytes)?;
        let lookup_len = u64::from_le_bytes(lookup_len_bytes);

        if lookup_len <= 1 {
            // TODO lookup has only 1 or no elements
            // requires special handling as loop below only works for the general case
        }

        let mut lookup = Vec::with_capacity(lookup_len as usize);

        let mut next_addr_bytes = [0u8; 8];
        read(f, &mut next_addr_bytes)?;
        let mut next_addr = u64::from_le_bytes(next_addr_bytes);
        for i in 0..lookup_len {
            let addr = next_addr;
            next_addr = if i == lookup_len - 1 {
                addr_original_paths
            } else {
                let mut next_addr_bytes = [0u8; 8];
                read(f, &mut next_addr_bytes)?;
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

            let len = (next_addr - addr) as usize;
            ris_log::trace!("asset {} {} {}", i, addr, len);
            let asset_entry = AssetEntry { addr, len };

            lookup.push(asset_entry);
        }

        Ok(Self { file, lookup })
    }

    pub fn load(&mut self, id: usize) -> Response {
        let entry = self.lookup.get(id).ok_or(LoadError::AssetNotFound)?;
        let f = &mut self.file;
        let mut bytes = vec![0u8; entry.len];
        seek(f, SeekFrom::Start(entry.addr))?;
        read(f, &mut bytes)?;
        Ok(bytes)
    }
}

fn seek(file: &mut File, pos: SeekFrom) -> Result<u64, LoadError> {
    file.seek(pos).map_err(|_| LoadError::FileReadFailed)
}

fn read(file: &mut File, buf: &mut [u8]) -> Result<usize, LoadError> {
    let read_bytes = file.read(buf).map_err(|_| LoadError::FileReadFailed)?;
    if read_bytes != buf.len() {
        ris_log::error!(
            "expected to read {} bytes but actually read {}",
            buf.len(),
            read_bytes
        );
        Err(LoadError::FileReadFailed)
    } else {
        Ok(read_bytes)
    }
}

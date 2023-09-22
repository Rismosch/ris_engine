use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::Write;

use ris_util::ris_error::RisError;

use crate::asset_loader::AssetId;

pub struct RisAsset{
    references: Vec<AssetId>,
    content: Vec<u8>,
}

pub enum RisLoaderError {
    NotRisAsset,
    IOError(RisError),
}

pub fn load(input: &mut (impl Read + Seek)) -> Result<RisAsset, RisLoaderError> {
    let mut magic_bytes = [0; crate::FAT_ADDR_SIZE];
    read(input, &mut magic_bytes)?;

    if magic_bytes[0] != 0x72 || // r
        magic_bytes[1] != 0x69 || // i
        magic_bytes[2] != 0x73 || // s
        magic_bytes[3] != 0x5f { // _
        return Err(RisLoaderError::NotRisAsset);
    }

    let mut reference_type = [0];
    read(input, &mut reference_type)?;

    ris_log::debug!("hi mom {}", reference_type[0]);

    match reference_type[0] {
        0 => {
            // directory
            let mut content_addr_bytes = [0; crate::ADDR_SIZE];
            read(input, &mut content_addr_bytes)?;
            let content_addr = u64::from_le_bytes(content_addr_bytes);

            let current_pos = seek(input, SeekFrom::Current(0))?;
            let reference_len = current_pos - content_addr;

            let mut reference_bytes = vec![0; reference_len as usize];
            read(input, &mut reference_bytes)?;

            let reference_string = String::from_utf8(reference_bytes)
                .map_err(|e| RisLoaderError::IOError(
                        ris_util::new_err!("failed to get reference string: {}", e)
                ))?;

            let references = reference_string
                .split('\0')
                .map(|x| crate::asset_loader::AssetId::Directory(String::from(x)))
                .collect();

            get content

            panic!("directory")
        },
        1 => {
            // compiled
            panic!("compiled")
        },
        byte => Err(RisLoaderError::IOError(ris_util::new_err!("invalid reference type {}", byte))),
    }
}

fn read(file: &mut impl Read, buf: &mut [u8]) -> Result<usize, RisLoaderError>{
    crate::util::read(file, buf).map_err(|e| RisLoaderError::IOError(e))
}

fn seek(file: &mut impl Seek, pos: SeekFrom) -> Result<u64, RisLoaderError> {
    crate::util::seek(file, pos).map_err(|e| RisLoaderError::IOError(e))
}


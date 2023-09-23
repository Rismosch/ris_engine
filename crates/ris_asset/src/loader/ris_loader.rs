use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;

use ris_util::ris_error::RisError;

use crate::AssetId;
use crate::RisAsset;

pub enum RisLoaderError {
    NotRisAsset,
    IOError(RisError),
}

pub fn load(input: &[u8]) -> Result<RisAsset, RisLoaderError> {
    let input = &mut std::io::Cursor::new(input);
    let mut magic = [0; crate::FAT_ADDR_SIZE];
    read(input, &mut magic)?;

    if magic[0] != 0x72 || // r
        magic[1] != 0x69 || // i
        magic[2] != 0x73 || // s
        magic[3] != 0x5f
    {
        // _
        return Err(RisLoaderError::NotRisAsset);
    }

    let mut reference_type = [0];
    read(input, &mut reference_type)?;

    match reference_type[0] {
        0 => {
            // directory
            let mut content_addr_bytes = [0; crate::ADDR_SIZE];
            read(input, &mut content_addr_bytes)?;
            let content_addr = u64::from_le_bytes(content_addr_bytes);

            let current_pos = seek(input, SeekFrom::Current(0))?;
            let reference_len = content_addr - current_pos;

            let mut reference_bytes = vec![0; reference_len as usize];
            read(input, &mut reference_bytes)?;

            let reference_string = String::from_utf8(reference_bytes).map_err(|e| {
                RisLoaderError::IOError(ris_util::new_err!("failed to get reference string: {}", e))
            })?;

            let references = reference_string
                .split('\0')
                .map(|x| AssetId::Directory(String::from(x)))
                .collect();

            let content_addr = seek(input, SeekFrom::Current(0))?;
            let stream_len = seek(input, SeekFrom::End(0))?;
            seek(input, SeekFrom::Start(0))?;
            let content_len = stream_len - content_addr;
            let mut content = vec![0; content_len as usize];
            read(input, &mut content)?;

            Ok(RisAsset {
                magic,
                references,
                content,
            })
        }
        1 => {
            // compiled
            panic!("compiled")
        }
        byte => Err(RisLoaderError::IOError(ris_util::new_err!(
            "invalid reference type {}",
            byte
        ))),
    }
}

fn read(file: &mut impl Read, buf: &mut [u8]) -> Result<usize, RisLoaderError> {
    crate::util::read(file, buf).map_err(RisLoaderError::IOError)
}

fn seek(file: &mut impl Seek, pos: SeekFrom) -> Result<u64, RisLoaderError> {
    crate::util::seek(file, pos).map_err(RisLoaderError::IOError)
}

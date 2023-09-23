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

pub fn load(input: &mut (impl Read + Seek)) -> Result<RisAsset, RisLoaderError> {
    let mut magic = [0; crate::FAT_ADDR_SIZE];
    read(input, &mut magic)?;

    if magic[0] != 0x72 || // r
        magic[1] != 0x69 || // i
        magic[2] != 0x73 || // s
        magic[3] != 0x5f
    {
        return Err(RisLoaderError::NotRisAsset);
    }

    let mut reference_type = [0];
    read(input, &mut reference_type)?;

    let references = match reference_type[0] {
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

            reference_string
                .split('\0')
                .map(|x| AssetId::Directory(String::from(x)))
                .collect()
        }
        1 => {
            // compiled
            let mut reference_count_bytes = [0; 4];
            read(input, &mut reference_count_bytes)?;
            let reference_count = u32::from_le_bytes(reference_count_bytes);

            let mut references = Vec::with_capacity(reference_count as usize);
            for _ in 0..reference_count {
                let mut reference_bytes = [0; 4];
                read(input, &mut reference_bytes)?;
                let reference_id = u32::from_le_bytes(reference_bytes);
                let reference = AssetId::Compiled(reference_id as usize);

                references.push(reference);
            }

            references
        }
        byte => return Err(RisLoaderError::IOError(ris_util::new_err!(
            "invalid reference type {}",
            byte
        ))),
    };

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

fn read(file: &mut impl Read, buf: &mut [u8]) -> Result<usize, RisLoaderError> {
    crate::util::read(file, buf).map_err(RisLoaderError::IOError)
}

fn seek(file: &mut impl Seek, pos: SeekFrom) -> Result<u64, RisLoaderError> {
    crate::util::seek(file, pos).map_err(RisLoaderError::IOError)
}

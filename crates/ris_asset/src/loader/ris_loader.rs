use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;

use ris_util::ris_error::RisError;

use crate::AssetId;
use crate::RisAssetData;

pub fn load(input: &mut (impl Read + Seek)) -> Result<Option<RisAssetData>, RisError> {
    let mut magic = [0; crate::FAT_ADDR_SIZE];
    crate::util::read(input, &mut magic)?;

    if magic[0] != 0x72 || // r
        magic[1] != 0x69 || // i
        magic[2] != 0x73 || // s
        magic[3] != 0x5f
    {
        return Ok(None);
    }

    let mut reference_type = [0];
    crate::util::read(input, &mut reference_type)?;

    let references = match reference_type[0] {
        0 => {
            // directory
            let mut content_addr_bytes = [0; crate::ADDR_SIZE];
            crate::util::read(input, &mut content_addr_bytes)?;
            let content_addr = u64::from_le_bytes(content_addr_bytes);

            let current_pos = crate::util::seek(input, SeekFrom::Current(0))?;
            let reference_len = content_addr - current_pos;

            let mut reference_bytes = vec![0; reference_len as usize];
            crate::util::read(input, &mut reference_bytes)?;

            let reference_string = ris_util::unroll!(
                String::from_utf8(reference_bytes),
                "failed to get reference string"
            )?;

            reference_string
                .split('\0')
                .map(|x| AssetId::Directory(String::from(x)))
                .collect()
        }
        1 => {
            // compiled
            let mut reference_count_bytes = [0; 4];
            crate::util::read(input, &mut reference_count_bytes)?;
            let reference_count = u32::from_le_bytes(reference_count_bytes);

            let mut references = Vec::with_capacity(reference_count as usize);
            for _ in 0..reference_count {
                let mut reference_bytes = [0; 4];
                crate::util::read(input, &mut reference_bytes)?;
                let reference_id = u32::from_le_bytes(reference_bytes);
                let reference = AssetId::Compiled(reference_id as usize);

                references.push(reference);
            }

            references
        }
        byte => return ris_util::result_err!(
            "invalid reference type {}",
            byte
        ),
    };

    let content_addr = crate::util::seek(input, SeekFrom::Current(0))?;

    Ok(Some(RisAssetData {
        magic,
        references,
        content_addr,
    }))
}


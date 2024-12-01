use std::io::Cursor;
use std::io::SeekFrom;

use ris_error::RisResult;
use ris_io::FatPtr;

use crate::AssetId;

// # File Format
//
// encoding: little-endian
//
// - [u8; 16]: magic (the first 4 u8 are always `ris_`, the other 12 indicate the type of asset)
// - u8 (boolean): is_compiled
//  - if is_compiled:
//   - u32: asset_count
//   - [u32; asset_count]: compiled AssetIds
//  - else:
//   - FatPtr: p_content
//   - [u8; ?]: directory AssetIds (utf8 encoded strings, seperated by `\0`)
// - [u8; ?]: content

#[derive(Debug)]
pub struct RisHeader {
    pub magic: [u8; 16],
    pub references: Vec<AssetId>,
    pub p_content: FatPtr,
}

impl RisHeader {
    pub fn load(bytes: &[u8]) -> RisResult<Option<Self>> {
        let input = &mut Cursor::new(bytes);
        let mut magic = [0; 16];
        ris_io::read(input, &mut magic)?;

        if magic[0] != 0x72 || // `r`
            magic[1] != 0x69 || // `i`
            magic[2] != 0x73 || // `s`
            magic[3] != 0x5f
        // `_`
        {
            return Ok(None);
        }

        let is_compiled = ris_io::read_bool(input)?;

        let (references, p_content) = if is_compiled {
            let reference_count = ris_io::read_uint(input)?;
            let mut references = Vec::with_capacity(reference_count);
            for _ in 0..reference_count {
                let id = ris_io::read_uint(input)?;
                let reference = AssetId::Compiled(id);
                references.push(reference);
            }

            let content_begin = ris_io::seek(input, SeekFrom::Current(0))?;
            let content_end = ris_io::seek(input, SeekFrom::End(0))?;
            let p_content = FatPtr::begin_end(content_begin, content_end)?;

            (references, p_content)
        } else {
            let p_content = ris_io::read_fat_ptr(input)?;

            let references_begin = ris_io::seek(input, SeekFrom::Current(0))?;
            let p_references = FatPtr::begin_end(references_begin, p_content.addr)?;
            let reference_bytes = ris_io::read_at(input, p_references)?;
            let reference_string = String::from_utf8(reference_bytes)?;
            let references = reference_string
                .split('\0')
                .map(|x| AssetId::Directory(x.to_string()))
                .collect::<Vec<_>>();

            (references, p_content)
        };

        Ok(Some(Self {
            magic,
            references,
            p_content,
        }))
    }
}

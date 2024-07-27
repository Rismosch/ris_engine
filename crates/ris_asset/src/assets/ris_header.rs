use std::io::Cursor;
use std::io::SeekFrom;

use ris_error::RisResult;
use ris_file::io::BinaryFormat;
use ris_file::io::FatPtr;

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
        ris_file::io::read_checked(input, &mut magic)?;

        if magic[0] != 0x72 || // `r`
            magic[1] != 0x69 || // `i`
            magic[2] != 0x73 || // `s`
            magic[3] != 0x5f
        // `_`
        {
            return Ok(None);
        }

        let is_compiled = ris_file::io::read_bool(input)?;

        let (references, p_content) = if is_compiled {
            let references = ris_file::io::read_array::<Reference>(input)?
                .iter()
                .map(|x| Ok(AssetId::Compiled(x.0.try_into()?)))
                .collect::<RisResult<Vec<_>>>()?;

            let content_begin = ris_file::io::seek(input, SeekFrom::Current(0))?;
            let content_end = ris_file::io::seek(input, SeekFrom::End(0))?;
            let p_content = FatPtr::begin_end(content_begin, content_end)?;

            (references, p_content)
        } else {
            let p_content = ris_file::io::read_fat_ptr(input)?;

            let references_begin = ris_file::io::seek(input, SeekFrom::Current(0))?;
            let p_references = FatPtr::begin_end(references_begin, p_content.addr)?;

            let references = ris_file::io::read_strings(input, p_references)?
                .into_iter()
                .map(AssetId::Directory)
                .collect();

            (references, p_content)
        };

        Ok(Some(Self {
            magic,
            references,
            p_content,
        }))
    }
}

pub struct Reference(pub u32);

impl BinaryFormat for Reference {
    fn serialized_length() -> usize {
        4
    }

    fn serialize(&self) -> std::io::Result<Vec<u8>> {
        Ok(self.0.to_le_bytes().to_vec())
    }

    fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        if buf.len() != 4 {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidInput));
        }

        Ok(Self(u32::from_le_bytes([buf[0], buf[1], buf[2], buf[3]])))
    }
}

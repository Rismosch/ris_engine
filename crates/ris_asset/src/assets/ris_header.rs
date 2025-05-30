use std::io::Cursor;
use std::io::SeekFrom;

use ris_asset_data::asset_id::AssetId;
use ris_error::RisResult;
use ris_io::path::SanitizeInfo;

// # File Format
//
// encoding: little-endian
//
// - [u8; 16]: magic (the first 4 u8 are always `ris_`, the other 12 indicate the type of asset)
// - u8 (boolean): is_compiled
// - u32: reference_count
//  - if is_compiled:
//   - [u32; asset_count]: compiled AssetIds
//  - else:
//   - [u32; sized String]: directory AssetIds
// - [u8; ?]: content

#[derive(Debug)]
pub struct RisHeader {
    pub magic: [u8; 16],
    pub references: Vec<AssetId>,
}

impl RisHeader {
    pub fn new(magic: [u8; 16], references: Vec<AssetId>) -> Self {
        Self { magic, references }
    }

    pub fn serialize(&self, content: &[u8]) -> RisResult<Vec<u8>> {
        let Self { magic, references } = self;

        if magic[0] != 0x72 || // `r`
            magic[1] != 0x69 || // `i`
            magic[2] != 0x73 || // `s`
            magic[3] != 0x5f
        // `_`
        {
            return ris_error::new_result!("not a ris asset");
        }

        let mut stream = Cursor::new(Vec::new());
        let s = &mut stream;

        ris_io::write(s, magic)?;

        let is_compiled = matches!(references.iter().next(), Some(AssetId::Index(_)));

        ris_io::write_bool(s, is_compiled)?;
        ris_io::write_uint(s, references.len())?;
        for reference in references.iter() {
            match reference {
                AssetId::Index(id) if is_compiled => ris_io::write_uint(s, *id)?,
                AssetId::Path(id) if !is_compiled => {
                    let sanitized_id = ris_io::path::sanitize(
                        id,
                        SanitizeInfo::RemoveInvalidCharsAndReplaceSlashes,
                    );
                    ris_io::write_string(s, sanitized_id)?
                }
                _ => {
                    return ris_error::new_result!(
                        "all references must be the same enum variant. is_compiled: {}",
                        is_compiled
                    )
                }
            };
        }

        ris_io::write(s, content)?;

        let bytes = stream.into_inner();
        Ok(bytes)
    }

    pub fn deserialize(bytes: &[u8]) -> RisResult<Option<(Self, &[u8])>> {
        let s = &mut Cursor::new(bytes);
        let mut magic = [0; 16];
        ris_io::read(s, &mut magic)?;

        if magic[0] != 0x72 || // `r`
            magic[1] != 0x69 || // `i`
            magic[2] != 0x73 || // `s`
            magic[3] != 0x5f
        // `_`
        {
            return Ok(None);
        }

        let is_compiled = ris_io::read_bool(s)?;
        let reference_count = ris_io::read_uint(s)?;
        let mut references = Vec::with_capacity(reference_count);
        for _ in 0..reference_count {
            let reference = if is_compiled {
                let id = ris_io::read_uint(s)?;
                AssetId::Index(id)
            } else {
                let id = ris_io::read_string(s)?;
                AssetId::Path(id)
            };

            references.push(reference);
        }

        let header = Self { magic, references };

        let content_begin = ris_io::seek(s, SeekFrom::Current(0))?.try_into()?;
        let content_end = ris_io::seek(s, SeekFrom::End(0))?.try_into()?;
        let content = &bytes[content_begin..content_end];

        Ok(Some((header, content)))
    }

    //pub fn p_content(&self) -> FatPtr {
    //    self.p_content
    //}

    //pub fn content<'a>(&'a self, bytes: &'a [u8]) -> RisResult<&'a [u8]> {
    //    let start: usize = self.p_content.addr.try_into()?;
    //    let end: usize = self.p_content.end().try_into()?;
    //    let slice = &bytes[start..end];
    //    Ok(slice)
    //}

    pub fn assert_magic(&self, magic: [u8; 16]) -> RisResult<()> {
        let left = magic;
        let right = self.magic;
        if left == right {
            Ok(())
        } else {
            let left_formatted = Self::format_magic(left);
            let right_formatted = Self::format_magic(right);
            ris_error::new_result!(
                "magic assert failed:\nexpected {}\nbut was  {}",
                left_formatted,
                right_formatted,
            )
        }
    }

    pub fn format_magic(magic: [u8; 16]) -> String {
        let result = magic
            .iter()
            .map(|&x| format!("0x{:02X}", x))
            .collect::<Vec<_>>()
            .join(", ");

        format!("[{}]", result)
    }
}

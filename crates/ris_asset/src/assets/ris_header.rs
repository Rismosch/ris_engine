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
    pub p_content: FatPtr,
}

impl RisHeader {
    pub fn serialize(&self) -> RisResult<Vec<u8>> {
        let Self{
            magic,
            references,
            ..
        } = self;

        if magic[0] != 0x72 || // `r`
            magic[1] != 0x69 || // `i`
            magic[2] != 0x73 || // `s`
            magic[3] != 0x5f
        // `_`
        {
            return ris_error::new_result!("not a ris asset");
        }

        let mut stream = Cursor::new(Vec::new());
        let f = &mut stream;

        ris_io::write(f, magic)?;

        let is_compiled = match references.iter().next() {
            Some(AssetId::Compiled(_)) => true,
            _ => false,
        };

        ris_io::write_bool(f, is_compiled)?;
        ris_io::write_uint(f, references.len())?;
        for reference in references.iter() {
            match reference {
                AssetId::Compiled(id) if is_compiled => ris_io::write_uint(f, *id)?,
                AssetId::Directory(id) if !is_compiled => ris_io::write_string(f, id)?,
                _ => return ris_error::new_result!("all references must be the same enum variant. is_compiled: {}", is_compiled),
            };
        }

        let bytes = stream.into_inner();
        Ok(bytes)
    }

    pub fn load(bytes: &[u8]) -> RisResult<Option<Self>> {
        let f = &mut Cursor::new(bytes);
        let mut magic = [0; 16];
        ris_io::read(f, &mut magic)?;

        if magic[0] != 0x72 || // `r`
            magic[1] != 0x69 || // `i`
            magic[2] != 0x73 || // `s`
            magic[3] != 0x5f
        // `_`
        {
            return Ok(None);
        }

        let is_compiled = ris_io::read_bool(f)?;
        let reference_count = ris_io::read_uint(f)?;
        let mut references = Vec::with_capacity(reference_count);
        for _ in 0..reference_count {
            let reference = if is_compiled {
                let id = ris_io::read_uint(f)?;
                AssetId::Compiled(id)
            } else {
                let id = ris_io::read_string(f)?;
                AssetId::Directory(id)
            };

            references.push(reference);
        }

        let content_begin = ris_io::seek(f , SeekFrom::Current(0))?;
        let content_end = ris_io::seek(f, SeekFrom::End(0))?;
        let p_content = FatPtr::begin_end(content_begin, content_end)?;

        Ok(Some(Self {
            magic,
            references,
            p_content,
        }))
    }

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
        let result = magic.iter()
            .map(|&x| format!("0x{:02X}", x))
            .collect::<Vec<_>>()
            .join(", ");

        format!("[{}]", result)
    }
}

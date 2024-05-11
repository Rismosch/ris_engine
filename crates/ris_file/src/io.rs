use std::io::Read;
use std::io::Write;
use std::io::Seek;
use std::io::SeekFrom;

use ris_error::RisResult;

pub const ADDR_SIZE: usize = std::mem::size_of::<u64>();

pub trait BinaryFormat : Sized {
    fn serialized_length() -> usize;
    fn serialize(&self) -> RisResult<Vec<u8>>;
    fn deserialize(buf: &[u8]) -> RisResult<Self>;
}

#[derive(Default, Debug, Clone, Copy)]
pub struct FatPtr {
    pub addr: u64,
    pub len: u64,
}

impl FatPtr {
    pub fn begin_end(begin: u64, end: u64) -> RisResult<FatPtr> {
        if begin > end {
            ris_error::new_result!(
                "begin {} must be less or equal than end {}",
                begin,
                end,
            )
        } else {
            Ok(FatPtr{
                addr: begin,
                len: end - begin,
            })
        }
    }

    pub fn end(self) -> u64 {
        self.addr + self.len
    }
}

//
// seek
//

pub fn seek(stream: &mut impl Seek, pos: SeekFrom) -> RisResult<u64> {
    let new_pos = stream.seek(pos)?;
    Ok(new_pos)
}

//
// read
//

pub fn read_unchecked(stream: &mut impl Read, buf: &mut [u8]) -> RisResult<usize> {
    let bytes_read = stream.read(buf)?;
    Ok(bytes_read)
}

pub fn read(stream: &mut impl Read, buf: &mut [u8]) -> RisResult<()> {
    let read_bytes = read_unchecked(stream, buf)?;
    let buf_len = buf.len();
    if read_bytes == buf_len {
        Ok(())
    } else {
        ris_error::new_result!(
            "expected to read {} bytes but actually read {}",
            buf_len,
            read_bytes,
        )
    }
}

pub fn read_i32(stream: &mut impl Read) -> RisResult<i32> {
    let mut bytes = [0; 4];
    read(stream, &mut bytes)?;

    Ok(i32::from_le_bytes(bytes))
}

pub fn read_u32(stream: &mut impl Read) -> RisResult<u32> {
    let mut bytes = [0; 4];
    read(stream, &mut bytes)?;

    Ok(u32::from_le_bytes(bytes))
}

pub fn read_u64(stream: &mut impl Read) -> RisResult<u64> {
    let mut bytes = [0; 8];
    read(stream, &mut bytes)?;

    Ok(u64::from_le_bytes(bytes))
}

pub fn read_f32(stream: &mut impl Read) -> RisResult<f32> {
    let mut bytes = [0; 4];
    read(stream, &mut bytes)?;

    Ok(f32::from_le_bytes(bytes))
}

pub fn read_bool(stream: &mut impl Read) -> RisResult<bool> {
    let mut bytes = [0; 1];
    read(stream, &mut bytes)?;

    match bytes[0] {
        1 => Ok(true),
        0 => Ok(false),
        b => ris_error::new_result!("{} is not a valid bool", b),
    }
}

pub fn read_array<T: BinaryFormat>(stream: &mut impl Read) -> RisResult<Vec<T>> {
    let len = read_u32(stream)?;
    let mut buf = Vec::with_capacity(len.try_into()?);
    for _ in 0..len {
        let mut bytes = vec![0; T::serialized_length()];
        read(stream, &mut bytes)?;
        let deserialized = T::deserialize(&bytes)?;
        buf.push(deserialized);
    }

    Ok(buf)
}

pub fn read_fat_ptr(stream: &mut impl Read) -> RisResult<FatPtr> {
    let addr = read_u64(stream)?;
    let len = read_u64(stream)?;
    Ok(FatPtr {
        addr,
        len,
    })
}

pub fn read_unsized(stream: &mut (impl Read + Seek), ptr: FatPtr) -> RisResult<Vec<u8>> {
    let mut bytes = vec![0; ptr.len.try_into()?];
    seek(stream, SeekFrom::Start(ptr.addr))?;
    read(stream, &mut bytes)?;
    Ok(bytes)
}

pub fn read_strings(stream: &mut (impl Read + Seek), ptr: FatPtr) -> RisResult<Vec<String>> {
    let bytes = read_unsized(stream, ptr)?;
    let strings = String::from_utf8(bytes)?;

    let splits = strings
        .split('\0')
        .map(|x| x.to_string())
        .collect();

    Ok(splits)
}

//
// write
//

pub fn write_unchecked(stream: &mut impl Write, buf: &[u8]) -> RisResult<usize> {
    let written_bytes = stream.write(buf)?;
    Ok(written_bytes)
}

pub fn write(stream: &mut impl Write, buf: &[u8]) -> RisResult<()> {
    let written_bytes = stream.write(buf)?;
    let buf_len = buf.len();
    if written_bytes == buf_len {
        Ok(())
    } else {
        ris_error::new_result!(
            "expected to write {} bytes but actually wrote {}",
            buf_len,
            written_bytes,
        )
    }

}

pub fn write_i32(stream: &mut impl Write, value: i32) -> RisResult<()> {
    let bytes = value.to_le_bytes();
    write(stream, &bytes)
}

pub fn write_u32(stream: &mut impl Write, value: u32) -> RisResult<()> {
    let bytes = value.to_le_bytes();
    write(stream, &bytes)
}

pub fn write_u64(stream: &mut impl Write, value: u64) -> RisResult<()> {
    let bytes = value.to_le_bytes();
    write(stream, &bytes)
}

pub fn write_f32(stream: &mut impl Write, value: f32) -> RisResult<()> {
    let bytes = value.to_le_bytes();
    write(stream, &bytes)
}

pub fn write_bool(stream: &mut impl Write, value: bool) -> RisResult<()> {
    match value {
        true => write(stream, &[1]),
        false => write(stream, &[0]),
    }
}

pub fn write_array<T: BinaryFormat>(stream: &mut impl Write, value: &[T]) -> RisResult<()> {
    let len = value.len().try_into()?;
    write_u32(stream, len)?;
    for entry in value.iter() {
        let bytes = entry.serialize()?;
        write(stream, &bytes)?;
    }

    Ok(())
}

pub fn write_fat_ptr(stream: &mut impl Write, value: FatPtr) -> RisResult<()> {
    write_u64(stream, value.addr)?;
    write_u64(stream, value.len)
}

pub fn write_unsized(stream: &mut (impl Write + Seek), buf: &[u8]) -> RisResult<FatPtr> {
    let addr = seek(stream, SeekFrom::Current(0))?;
    write(stream, buf)?;
    Ok(FatPtr{
        addr,
        len: buf.len().try_into()?,
    })
}

pub fn write_strings(stream: &mut (impl Write + Seek), strings: &[&str]) -> RisResult<FatPtr> {
    let begin = seek(stream, SeekFrom::Current(0))?;
    if strings.len() >= 1 {
        let first = strings[0];
        let bytes = first.as_bytes();
        write(stream, bytes)?;
        for string in strings.iter().skip(1) {
            write(stream, &[0])?;
            let bytes = string.as_bytes();
            write(stream, bytes)?;
        }
    }
    let end = seek(stream, SeekFrom::Current(0))?;
    FatPtr::begin_end(begin, end)
}

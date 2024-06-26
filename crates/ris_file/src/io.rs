use std::io::Error;
use std::io::ErrorKind;
use std::io::Read;
use std::io::Result;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::Write;

pub const ADDR_SIZE: usize = std::mem::size_of::<u64>();

pub trait BinaryFormat: Sized {
    fn serialized_length() -> usize;
    fn serialize(&self) -> Result<Vec<u8>>;
    fn deserialize(buf: &[u8]) -> Result<Self>;
}

#[derive(Debug, Clone, Copy)]
pub struct FatPtr {
    pub addr: u64,
    pub len: u64,
}

impl FatPtr {
    pub fn null() -> FatPtr {
        Self { addr: 0, len: 0 }
    }

    pub fn begin_end(begin: u64, end: u64) -> Result<FatPtr> {
        if begin > end {
            Err(Error::from(ErrorKind::InvalidInput))
        } else {
            Ok(FatPtr {
                addr: begin,
                len: end - begin,
            })
        }
    }

    pub fn end(self) -> u64 {
        self.addr + self.len
    }

    pub fn is_null(self) -> bool {
        self.addr == 0 && self.len == 0
    }
}

//
// seek
//

pub fn seek(stream: &mut impl Seek, pos: SeekFrom) -> Result<u64> {
    stream.seek(pos)
}

//
// read
//

pub fn read_unchecked(stream: &mut impl Read, buf: &mut [u8]) -> Result<usize> {
    stream.read(buf)
}

pub fn read_checked(stream: &mut impl Read, buf: &mut [u8]) -> Result<usize> {
    let read_bytes = read_unchecked(stream, buf)?;
    let buf_len = buf.len();
    if read_bytes == buf_len {
        Ok(read_bytes)
    } else {
        Err(Error::from(ErrorKind::Other))
    }
}

pub fn read_unsized(stream: &mut (impl Read + Seek), ptr: FatPtr) -> Result<Vec<u8>> {
    let capacity = ptr
        .len
        .try_into()
        .map_err(|_| Error::from(ErrorKind::InvalidData))?;
    let mut bytes = vec![0; capacity];
    seek(stream, SeekFrom::Start(ptr.addr))?;
    read_checked(stream, &mut bytes)?;
    Ok(bytes)
}

pub fn read_u8(stream: &mut impl Read) -> Result<u8> {
    let mut bytes = [0];
    read_checked(stream, &mut bytes)?;
    Ok(bytes[0])
}

pub fn read_i32(stream: &mut impl Read) -> Result<i32> {
    let mut bytes = [0; 4];
    read_checked(stream, &mut bytes)?;

    Ok(i32::from_le_bytes(bytes))
}

pub fn read_u32(stream: &mut impl Read) -> Result<u32> {
    let mut bytes = [0; 4];
    read_checked(stream, &mut bytes)?;

    Ok(u32::from_le_bytes(bytes))
}

pub fn read_u64(stream: &mut impl Read) -> Result<u64> {
    let mut bytes = [0; 8];
    read_checked(stream, &mut bytes)?;

    Ok(u64::from_le_bytes(bytes))
}

pub fn read_f32(stream: &mut impl Read) -> Result<f32> {
    let mut bytes = [0; 4];
    read_checked(stream, &mut bytes)?;

    Ok(f32::from_le_bytes(bytes))
}

pub fn read_bool(stream: &mut impl Read) -> Result<bool> {
    let mut bytes = [0; 1];
    read_checked(stream, &mut bytes)?;

    match bytes[0] {
        1 => Ok(true),
        0 => Ok(false),
        _ => Err(Error::from(ErrorKind::InvalidData)),
    }
}

pub fn read_array<T: BinaryFormat>(stream: &mut impl Read) -> Result<Vec<T>> {
    let len = read_u32(stream)?;
    let capacity = len
        .try_into()
        .map_err(|_| Error::from(ErrorKind::InvalidData))?;
    let mut buf = Vec::with_capacity(capacity);
    for _ in 0..len {
        let mut bytes = vec![0; T::serialized_length()];
        read_checked(stream, &mut bytes)?;
        let deserialized = T::deserialize(&bytes)?;
        buf.push(deserialized);
    }

    Ok(buf)
}

pub fn read_fat_ptr(stream: &mut impl Read) -> Result<FatPtr> {
    let addr = read_u64(stream)?;
    let len = read_u64(stream)?;
    Ok(FatPtr { addr, len })
}

pub fn read_strings(stream: &mut (impl Read + Seek), ptr: FatPtr) -> Result<Vec<String>> {
    let bytes = read_unsized(stream, ptr)?;
    if bytes.is_empty() {
        Ok(Vec::new())
    } else {
        let strings = String::from_utf8(bytes).map_err(|_| Error::from(ErrorKind::InvalidData))?;

        let splits = strings.split('\0').map(|x| x.to_string()).collect();

        Ok(splits)
    }
}

//
// write
//

pub fn write_unchecked(stream: &mut impl Write, buf: &[u8]) -> Result<usize> {
    stream.write(buf)
}

pub fn write_checked(stream: &mut impl Write, buf: &[u8]) -> Result<usize> {
    let written_bytes = stream.write(buf)?;
    let buf_len = buf.len();
    if written_bytes == buf_len {
        Ok(written_bytes)
    } else {
        Err(Error::from(ErrorKind::Other))
    }
}

pub fn write_unsized(stream: &mut (impl Write + Seek), buf: &[u8]) -> Result<FatPtr> {
    let addr = seek(stream, SeekFrom::Current(0))?;
    let len = buf
        .len()
        .try_into()
        .map_err(|_| Error::from(ErrorKind::InvalidData))?;
    write_checked(stream, buf)?;
    Ok(FatPtr { addr, len })
}

pub fn write_u8(stream: &mut impl Write, value: u8) -> Result<usize> {
    write_checked(stream, &[value])
}

pub fn write_i32(stream: &mut impl Write, value: i32) -> Result<usize> {
    let bytes = value.to_le_bytes();
    write_checked(stream, &bytes)
}

pub fn write_u32(stream: &mut impl Write, value: u32) -> Result<usize> {
    let bytes = value.to_le_bytes();
    write_checked(stream, &bytes)
}

pub fn write_u64(stream: &mut impl Write, value: u64) -> Result<usize> {
    let bytes = value.to_le_bytes();
    write_checked(stream, &bytes)
}

pub fn write_f32(stream: &mut impl Write, value: f32) -> Result<usize> {
    let bytes = value.to_le_bytes();
    write_checked(stream, &bytes)
}

pub fn write_bool(stream: &mut impl Write, value: bool) -> Result<usize> {
    match value {
        true => write_checked(stream, &[1]),
        false => write_checked(stream, &[0]),
    }
}

pub fn write_array<T: BinaryFormat>(stream: &mut impl Write, value: &[T]) -> Result<usize> {
    let len = value
        .len()
        .try_into()
        .map_err(|_| Error::from(ErrorKind::InvalidData))?;
    let mut written_bytes = write_u32(stream, len)?;
    for entry in value.iter() {
        let bytes = entry.serialize()?;
        written_bytes += write_checked(stream, &bytes)?;
    }

    Ok(written_bytes)
}

pub fn write_fat_ptr(stream: &mut impl Write, value: FatPtr) -> Result<usize> {
    let mut written_bytes = 0;
    written_bytes += write_u64(stream, value.addr)?;
    written_bytes += write_u64(stream, value.len)?;

    Ok(written_bytes)
}

pub fn write_strings(stream: &mut (impl Write + Seek), strings: &[&str]) -> Result<FatPtr> {
    let begin = seek(stream, SeekFrom::Current(0))?;
    if !strings.is_empty() {
        let first = strings[0];
        let bytes = first.as_bytes();
        write_checked(stream, bytes)?;
        for string in strings.iter().skip(1) {
            write_checked(stream, &[0])?;
            let bytes = string.as_bytes();
            write_checked(stream, bytes)?;
        }
    }
    let end = seek(stream, SeekFrom::Current(0))?;
    FatPtr::begin_end(begin, end)
}

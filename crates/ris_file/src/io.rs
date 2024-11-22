use std::io::Error;
use std::io::ErrorKind;
use std::io::Read;
use std::io::Result;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::Write;

pub const ADDR_SIZE: usize = std::mem::size_of::<u64>();

/// represents a sized memory location. used in combination with stream io operations.
///
/// Example:
///
/// FatPtr {
///     addr: 1,
///     len 2,
/// }
///
/// ```
/// | Byte 0 | Byte 1 | Byte 2 | Byte 3 | Byte 4 |
///            addr              end()
/// ```
///
/// The `FatPtr` in this example refers to Byte 1 and Byte 2.
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

/// thin wrapper around `std::io::Seek::seek()`.
pub fn seek(stream: &mut impl Seek, pos: SeekFrom) -> Result<u64> {
    stream.seek(pos)
}

//
// write
//

/// writes and advances the stream. if not all bytes were written, an error is returned. returns a
/// `FatPtr` to the bytes written.
pub fn write(stream: &mut (impl Write + Seek), buf: &[u8]) -> Result<FatPtr> {
    let addr = seek(stream, SeekFrom::Current(0))?;

    let written_bytes = stream.write(buf)?;
    let buf_len = buf.len();
    if written_bytes != buf_len {
        return Err(Error::from(ErrorKind::Other));
    }

    let len = buf
        .len()
        .try_into()
        .map_err(|_| Error::from(ErrorKind::InvalidData))?;

    let fat_ptr = FatPtr {
        addr,
        len,
    };

    Ok(fat_ptr)
}

/// writes a single `u8` and advances the stream. returns a `FatPtr` to the byte written.
pub fn write_u8(stream: &mut (impl Write + Seek), value: u8) -> Result<FatPtr> {
    write(stream, &[value])
}


/// converts an `isize` to a `i32`, writes and advances the stream. returns a `FatPtr` to the bytes
/// written.
pub fn write_int(stream: &mut (impl Write + Seek), value: isize) -> Result<FatPtr> {
    let int = i32::try_from(value).map_err(|_| Error::from(ErrorKind::InvalidData))?;
    let bytes = int.to_le_bytes();
    write(stream, &bytes)
}

/// converts an `usize` to a `u32`, writes and advances the stream. returns a `FatPtr` to the bytes
/// written.
pub fn write_uint(stream: &mut (impl Write + Seek), value: usize) -> Result<FatPtr> {
    let int = u32::try_from(value).map_err(|_| Error::from(ErrorKind::InvalidData))?;
    let bytes = int.to_le_bytes();
    write(stream, &bytes)
}

/// writes an `u64` and advances the stream. returns a `FatPtr` to the byte written.
pub fn write_u64(stream: &mut (impl Write + Seek), value: u64) -> Result<FatPtr> {
    let bytes = value.to_le_bytes();
    write(stream, &bytes)
}

/// writes a `f64` and advances the stream. returns a `FatPtr` to the byte written.
pub fn write_f32(stream: &mut (impl Write + Seek), value: f32) -> Result<FatPtr> {
    let bytes = value.to_le_bytes();
    write(stream, &bytes)
}

/// writes an `1` if `value` is `true`, `0` otherwise. it advances the stream. returns a `FatPtr` to the byte written.
pub fn write_bool(stream: &mut (impl Write + Seek), value: bool) -> Result<FatPtr> {
    match value {
        true => write(stream, &[1]),
        false => write(stream, &[0]),
    }
}

/// writes a `FatPtr` and advances the stream. returns a `FatPtr` to the byte written.
pub fn write_fat_ptr(stream: &mut (impl Write + Seek), value: FatPtr) -> Result<FatPtr> {
    let p_addr = write_u64(stream, value.addr)?;
    let p_len = write_u64(stream, value.len)?;
    let addr = p_addr.addr;
    let len = p_addr.len + p_len.len;
    let fat_ptr = FatPtr{addr, len};
    Ok(fat_ptr)
}

/// writes a string and advances the stream. it does so by writing it's len as an `u32`, followed
/// by it's UTF-8 encoded bytes.
pub fn write_string(stream: &mut (impl Write + Seek), string: impl AsRef<str>) -> Result<FatPtr> {
    let string = string.as_ref();
    let begin = seek(stream, SeekFrom::Current(0))?;
    write_uint(stream, string.len())?;
    let bytes = string.as_bytes();
    let fat_ptr = write(stream, bytes)?;
    let end = fat_ptr.end();
    FatPtr::begin_end(begin, end)
}

//
// read
//

/// reads and advances the stream. if not all expected bytes were read, an error is returned.
pub fn read(stream: &mut impl Read, buf: &mut [u8]) -> Result<()> {
    let read_bytes = stream.read(buf)?;
    let buf_len = buf.len();
    if read_bytes == buf_len {
        Ok(())
    } else {
        Err(Error::from(ErrorKind::Other))
    }
}

/// seeks to, reads the bytes at `ptr` and advances the stream.
pub fn read_at(stream: &mut (impl Read + Seek), ptr: FatPtr) -> Result<Vec<u8>> {
    let capacity = ptr
        .len
        .try_into()
        .map_err(|_| Error::from(ErrorKind::InvalidData))?;
    let mut bytes = vec![0; capacity];
    seek(stream, SeekFrom::Start(ptr.addr))?;
    read(stream, &mut bytes)?;
    Ok(bytes)
}

/// reads a single `u8` and advances the stream.
pub fn read_u8(stream: &mut impl Read) -> Result<u8> {
    let mut bytes = [0];
    read(stream, &mut bytes)?;
    Ok(bytes[0])
}

/// reads an `i32`, converts it to `isize` and advances the stream.
pub fn read_int(stream: &mut impl Read) -> Result<isize> {
    let mut bytes = [0; 4];
    read(stream, &mut bytes)?;
    let int = i32::from_le_bytes(bytes);
    let result = isize::try_from(int).map_err(|_| Error::from(ErrorKind::InvalidData))?;
    Ok(result)
}

/// reads an `u32`, converts it to `usize` and advances the stream.
pub fn read_uint(stream: &mut impl Read) -> Result<usize> {
    let mut bytes = [0; 4];
    read(stream, &mut bytes)?;
    let int = i32::from_le_bytes(bytes);
    let result = usize::try_from(int).map_err(|_| Error::from(ErrorKind::InvalidData))?;
    Ok(result)
}

/// reads an `u64` and advances the stream.
pub fn read_u64(stream: &mut impl Read) -> Result<u64> {
    let mut bytes = [0; 8];
    read(stream, &mut bytes)?;

    Ok(u64::from_le_bytes(bytes))
}

/// reads an `f32` and advances the stream.
pub fn read_f32(stream: &mut impl Read) -> Result<f32> {
    let mut bytes = [0; 4];
    read(stream, &mut bytes)?;

    Ok(f32::from_le_bytes(bytes))
}

/// reads an `u8` and advances the stream. returns `true` the read value is `1`, `false` if the read value is `0`, and an
/// error otherwise.
pub fn read_bool(stream: &mut impl Read) -> Result<bool> {
    let mut bytes = [0; 1];
    read(stream, &mut bytes)?;

    match bytes[0] {
        1 => Ok(true),
        0 => Ok(false),
        _ => Err(Error::from(ErrorKind::InvalidData)),
    }
}

/// reads a `FatPtr` and advances the stream.
pub fn read_fat_ptr(stream: &mut impl Read) -> Result<FatPtr> {
    let addr = read_u64(stream)?;
    let len = read_u64(stream)?;
    Ok(FatPtr { addr, len })
}

/// reads a string and advances the stream. it does so by reading a `u32`, and then reads that many
/// UTF-8 encoded bytes.
pub fn read_string(stream: &mut (impl Read + Seek)) -> Result<String> {
    let length = read_uint(stream)?;
    let mut bytes = vec![0; length];
    read(stream, &mut bytes)?;
    let string = String::from_utf8(bytes).map_err(|_| Error::from(ErrorKind::InvalidData))?;
    Ok(string)
}


use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::Write;

pub const ADDR_SIZE: usize = std::mem::size_of::<u64>();

//
// errors
//

ris_error::declare_error!(BeginWasBiggerThanEnd, "begin was bigger than end");
ris_error::declare_error!(
    WrittenBytesDoNotMatchBufLen,
    "written bytes do not match buf len"
);
ris_error::declare_error!(ReadBytesDoNotMatchBufLen, "read bytes do not match buf len");

#[derive(Debug, Default)]
pub struct ConversionError {
    type_name_from: &'static str,
    type_name_to: &'static str,
}

impl ConversionError {
    pub fn new<TFrom, TTo>() -> ConversionError {
        ConversionError {
            type_name_from: std::any::type_name::<TFrom>(),
            type_name_to: std::any::type_name::<TTo>(),
        }
    }
}

#[derive(Debug)]
pub enum WriteError {
    IO(std::io::Error),
    WrittenBytesDoNotMatchBufLen(WrittenBytesDoNotMatchBufLen),
    ConversionError(ConversionError),
    FatPtrError(BeginWasBiggerThanEnd),
}

#[derive(Debug)]
pub enum ReadError {
    IO(std::io::Error),
    ReadBytesDoNotMatchBufLen(ReadBytesDoNotMatchBufLen),
    ConversionError(ConversionError),
}

impl From<std::io::Error> for WriteError {
    fn from(value: std::io::Error) -> Self {
        WriteError::IO(value)
    }
}

impl From<WrittenBytesDoNotMatchBufLen> for WriteError {
    fn from(value: WrittenBytesDoNotMatchBufLen) -> Self {
        WriteError::WrittenBytesDoNotMatchBufLen(value)
    }
}

impl From<ConversionError> for WriteError {
    fn from(value: ConversionError) -> Self {
        WriteError::ConversionError(value)
    }
}

impl From<BeginWasBiggerThanEnd> for WriteError {
    fn from(value: BeginWasBiggerThanEnd) -> Self {
        WriteError::FatPtrError(value)
    }
}

impl From<std::io::Error> for ReadError {
    fn from(value: std::io::Error) -> Self {
        ReadError::IO(value)
    }
}

impl From<ReadBytesDoNotMatchBufLen> for ReadError {
    fn from(value: ReadBytesDoNotMatchBufLen) -> Self {
        ReadError::ReadBytesDoNotMatchBufLen(value)
    }
}

impl From<ConversionError> for ReadError {
    fn from(value: ConversionError) -> Self {
        ReadError::ConversionError(value)
    }
}

impl std::fmt::Display for ConversionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "failed to convert from {} to {}",
            self.type_name_from, self.type_name_to
        )
    }
}

impl std::fmt::Display for WriteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            WriteError::IO(ref e) => write!(f, "{}", e),
            WriteError::WrittenBytesDoNotMatchBufLen(ref e) => write!(f, "{}", e),
            WriteError::ConversionError(ref e) => write!(f, "{}", e),
            WriteError::FatPtrError(ref e) => write!(f, "{}", e),
        }
    }
}

impl std::fmt::Display for ReadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ReadError::IO(ref e) => write!(f, "{}", e),
            ReadError::ReadBytesDoNotMatchBufLen(ref e) => write!(f, "{}", e),
            ReadError::ConversionError(ref e) => write!(f, "{}", e),
        }
    }
}

impl std::error::Error for ConversionError {}
impl std::error::Error for WriteError {}
impl std::error::Error for ReadError {}

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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FatPtr {
    pub addr: u64,
    pub len: u64,
}

impl FatPtr {
    pub fn null() -> FatPtr {
        Self { addr: 0, len: 0 }
    }

    pub fn begin_end(begin: u64, end: u64) -> Result<FatPtr, BeginWasBiggerThanEnd> {
        if begin > end {
            Err(BeginWasBiggerThanEnd)
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
pub fn seek(stream: &mut impl Seek, pos: SeekFrom) -> Result<u64, std::io::Error> {
    stream.seek(pos)
}

//
// write
//

/// writes and advances the stream. if not all bytes were written, an error is returned. returns a
/// `FatPtr` to the bytes written.
pub fn write(stream: &mut (impl Write + Seek), buf: &[u8]) -> Result<FatPtr, WriteError> {
    let addr = seek(stream, SeekFrom::Current(0))?;

    let written_bytes = stream.write(buf)?;
    let buf_len = buf.len();
    if written_bytes != buf_len {
        return Err(WrittenBytesDoNotMatchBufLen.into());
    }

    let len = buf
        .len()
        .try_into()
        .map_err(|_| ConversionError::new::<usize, u64>())?;

    let fat_ptr = FatPtr { addr, len };

    Ok(fat_ptr)
}

/// writes a single `u8` and advances the stream. returns a `FatPtr` to the byte written.
pub fn write_u8(stream: &mut (impl Write + Seek), value: u8) -> Result<FatPtr, WriteError> {
    write(stream, &[value])
}

/// converts an `isize` to a `i32`, writes and advances the stream. returns a `FatPtr` to the bytes
/// written.
///
/// deliberately not called `write_isize`, because no `isize` is being written.
pub fn write_int(stream: &mut (impl Write + Seek), value: isize) -> Result<FatPtr, WriteError> {
    let int = i32::try_from(value).map_err(|_| ConversionError::new::<isize, i32>())?;
    let bytes = int.to_le_bytes();
    write(stream, &bytes)
}

/// converts an `usize` to a `u32`, writes and advances the stream. returns a `FatPtr` to the bytes
/// written.
///
/// deliberately not called `write_usize`, because no `usize` is being written.
pub fn write_uint(stream: &mut (impl Write + Seek), value: usize) -> Result<FatPtr, WriteError> {
    let int = u32::try_from(value).map_err(|_| ConversionError::new::<isize, u32>())?;
    let bytes = int.to_le_bytes();
    write(stream, &bytes)
}

/// writes an `u16` and advances the stream. returns a `FatPtr` to the byte written.
pub fn write_u16(stream: &mut (impl Write + Seek), value: u16) -> Result<FatPtr, WriteError> {
    let bytes = value.to_le_bytes();
    write(stream, &bytes)
}

/// writes an `u32` and advances the stream. returns a `FatPtr` to the byte written.
pub fn write_u32(stream: &mut (impl Write + Seek), value: u32) -> Result<FatPtr, WriteError> {
    let bytes = value.to_le_bytes();
    write(stream, &bytes)
}

/// writes an `u64` and advances the stream. returns a `FatPtr` to the byte written.
pub fn write_u64(stream: &mut (impl Write + Seek), value: u64) -> Result<FatPtr, WriteError> {
    let bytes = value.to_le_bytes();
    write(stream, &bytes)
}

/// writes an `i32` and advances the stream. returns a `FatPtr` to the byte written.
pub fn write_i32(stream: &mut (impl Write + Seek), value: i32) -> Result<FatPtr, WriteError> {
    let bytes = value.to_le_bytes();
    write(stream, &bytes)
}

/// writes a `f64` and advances the stream. returns a `FatPtr` to the byte written.
pub fn write_f32(stream: &mut (impl Write + Seek), value: f32) -> Result<FatPtr, WriteError> {
    let bytes = value.to_le_bytes();
    write(stream, &bytes)
}

/// writes an `1` if `value` is `true`, `0` otherwise. it advances the stream. returns a `FatPtr` to the byte written.
pub fn write_bool(stream: &mut (impl Write + Seek), value: bool) -> Result<FatPtr, WriteError> {
    match value {
        true => write(stream, &[1]),
        false => write(stream, &[0]),
    }
}

/// writes a `FatPtr` and advances the stream. returns a `FatPtr` to the byte written.
pub fn write_fat_ptr(
    stream: &mut (impl Write + Seek),
    value: FatPtr,
) -> Result<FatPtr, WriteError> {
    let p_addr = write_u64(stream, value.addr)?;
    let p_len = write_u64(stream, value.len)?;
    let addr = p_addr.addr;
    let len = p_addr.len + p_len.len;
    let fat_ptr = FatPtr { addr, len };
    Ok(fat_ptr)
}

/// writes a string and advances the stream. it does so by writing it's len as an `u32`, followed
/// by it's UTF-8 encoded bytes.
pub fn write_string(
    stream: &mut (impl Write + Seek),
    string: impl AsRef<str>,
) -> Result<FatPtr, WriteError> {
    let string = string.as_ref();
    let begin = seek(stream, SeekFrom::Current(0))?;
    write_uint(stream, string.len())?;
    let bytes = string.as_bytes();
    let fat_ptr = write(stream, bytes)?;
    let end = fat_ptr.end();
    let result = FatPtr::begin_end(begin, end)?;
    Ok(result)
}

//
// read
//

/// reads and advances the stream. if not all expected bytes were read, an error is returned.
pub fn read(stream: &mut impl Read, buf: &mut [u8]) -> Result<(), ReadError> {
    let read_bytes = stream.read(buf)?;
    let buf_len = buf.len();
    if read_bytes == buf_len {
        Ok(())
    } else {
        Err(ReadBytesDoNotMatchBufLen.into())
    }
}

// reads and advances the stream up to the end. returns all bytes that were read
pub fn read_to_end(stream: &mut (impl Read + Seek)) -> Result<Vec<u8>, ReadError> {
    let current = seek(stream, SeekFrom::Current(0))?;
    let end = seek(stream, SeekFrom::End(0))?;
    seek(stream, SeekFrom::Start(current))?;

    match usize::try_from(end - current) {
        Ok(len) => {
            let mut buf = vec![0; len];
            read(stream, &mut buf)?;
            Ok(buf)
        }
        Err(_) => Err(ConversionError::new::<u64, usize>().into()),
    }
}

/// seeks to, reads the bytes at `ptr` and advances the stream.
pub fn read_at(stream: &mut (impl Read + Seek), ptr: FatPtr) -> Result<Vec<u8>, ReadError> {
    let capacity = ptr
        .len
        .try_into()
        .map_err(|_| ConversionError::new::<u64, usize>())?;
    let mut bytes = vec![0; capacity];
    seek(stream, SeekFrom::Start(ptr.addr))?;
    read(stream, &mut bytes)?;
    Ok(bytes)
}

/// reads a single `u8` and advances the stream.
pub fn read_u8(stream: &mut impl Read) -> Result<u8, ReadError> {
    let mut bytes = [0];
    read(stream, &mut bytes)?;
    Ok(bytes[0])
}

/// reads an `i32`, converts it to `isize` and advances the stream.
///
/// deliberately not called `read_isize`, because no `usize` is being read.
pub fn read_int(stream: &mut impl Read) -> Result<isize, ReadError> {
    let mut bytes = [0; 4];
    read(stream, &mut bytes)?;
    let int = i32::from_le_bytes(bytes);
    let result = isize::try_from(int).map_err(|_| ConversionError::new::<i32, isize>())?;
    Ok(result)
}

/// reads an `u32`, converts it to `usize` and advances the stream.
///
/// deliberately not called `read_usize`, because no `usize` is being read.
pub fn read_uint(stream: &mut impl Read) -> Result<usize, ReadError> {
    let mut bytes = [0; 4];
    read(stream, &mut bytes)?;
    let int = u32::from_le_bytes(bytes);
    let result = usize::try_from(int).map_err(|_| ConversionError::new::<u32, usize>())?;
    Ok(result)
}

/// reads an `u16` and advances the stream.
pub fn read_u16(stream: &mut impl Read) -> Result<u16, ReadError> {
    let mut bytes = [0; 2];
    read(stream, &mut bytes)?;

    Ok(u16::from_le_bytes(bytes))
}

/// reads an `u32` and advances the stream.
pub fn read_u32(stream: &mut impl Read) -> Result<u32, ReadError> {
    let mut bytes = [0; 4];
    read(stream, &mut bytes)?;

    Ok(u32::from_le_bytes(bytes))
}

/// reads an `u64` and advances the stream.
pub fn read_u64(stream: &mut impl Read) -> Result<u64, ReadError> {
    let mut bytes = [0; 8];
    read(stream, &mut bytes)?;

    Ok(u64::from_le_bytes(bytes))
}

/// reads an `i32` and advances the stream.
pub fn read_i32(stream: &mut impl Read) -> Result<i32, ReadError> {
    let mut bytes = [0; 4];
    read(stream, &mut bytes)?;

    Ok(i32::from_le_bytes(bytes))
}

/// reads an `f32` and advances the stream.
pub fn read_f32(stream: &mut impl Read) -> Result<f32, ReadError> {
    let mut bytes = [0; 4];
    read(stream, &mut bytes)?;

    Ok(f32::from_le_bytes(bytes))
}

/// reads an `u8` and advances the stream. returns `true` the read value is `1`, `false` if the read value is `0`, and an
/// error otherwise.
pub fn read_bool(stream: &mut impl Read) -> Result<bool, ReadError> {
    let mut bytes = [0; 1];
    read(stream, &mut bytes)?;

    match bytes[0] {
        1 => Ok(true),
        0 => Ok(false),
        _ => Err(ConversionError::new::<u8, bool>().into()),
    }
}

/// reads a `FatPtr` and advances the stream.
pub fn read_fat_ptr(stream: &mut impl Read) -> Result<FatPtr, ReadError> {
    let addr = read_u64(stream)?;
    let len = read_u64(stream)?;
    Ok(FatPtr { addr, len })
}

/// reads a string and advances the stream. it does so by reading a `u32`, and then reads that many
/// UTF-8 encoded bytes.
pub fn read_string(stream: &mut (impl Read + Seek)) -> Result<String, ReadError> {
    let length = read_uint(stream)?;
    let mut bytes = vec![0; length];
    read(stream, &mut bytes)?;
    let string = String::from_utf8(bytes).map_err(|_| ConversionError::new::<&[u8], String>())?;
    Ok(string)
}

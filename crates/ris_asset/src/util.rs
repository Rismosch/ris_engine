use std::fs::File;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::Write;

use ris_util::ris_error::RisError;

pub fn seek(file: &mut File, pos: SeekFrom) -> Result<u64, RisError> {
    ris_util::unroll!(file.seek(pos), "failed to seek")
}

pub fn read(file: &mut File, buf: &mut [u8]) -> Result<usize, RisError> {
    let read_bytes = ris_util::unroll!(file.read(buf), "failed to read")?;
    if read_bytes != buf.len() {
        ris_util::result_err!(
            "expected to read {} bytes but actually read {}",
            buf.len(),
            read_bytes,
        )
    } else {
        Ok(read_bytes)
    }
}

pub fn write(file: &mut File, buf: &[u8]) -> Result<usize, RisError> {
    let written_bytes = ris_util::unroll!(file.write(buf), "failed to write")?;
    if written_bytes != buf.len() {
        ris_util::result_err!(
            "expected to write {} bytes but actually wrote {}",
            buf.len(),
            written_bytes,
        )
    } else {
        Ok(written_bytes)
    }
}

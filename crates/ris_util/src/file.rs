use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::Write;

use crate as ris_util;
use crate::error::RisResult;

pub fn seek(file: &mut impl Seek, pos: SeekFrom) -> RisResult<u64> {
    ris_util::unroll!(file.seek(pos), "failed to seek")
}

pub fn read(file: &mut impl Read, buf: &mut [u8]) -> RisResult<usize> {
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

pub fn write(file: &mut impl Write, buf: &[u8]) -> RisResult<usize> {
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

pub fn bytes_equal(left: &[u8], right: &[u8]) -> bool {
    if left.len() != right.len() {
        return false;
    }

    for i in 0..left.len() {
        let left = left[i];
        let right = right[i];

        if left != right {
            return false;
        }
    }

    true
}

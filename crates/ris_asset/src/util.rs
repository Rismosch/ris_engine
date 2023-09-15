use std::collections::VecDeque;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

use ris_util::ris_error::RisError;

pub fn seek(file: &mut impl Seek, pos: SeekFrom) -> Result<u64, RisError> {
    ris_util::unroll!(file.seek(pos), "failed to seek")
}

pub fn read(file: &mut impl Read, buf: &mut [u8]) -> Result<usize, RisError> {
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

pub fn write(file: &mut impl Write, buf: &[u8]) -> Result<usize, RisError> {
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

pub fn create_dir_all(path: &Path) -> std::io::Result<()> {
    let mut to_create = VecDeque::new();
    
    let mut parent = path.parent();
    while let Some(directory) = parent {
        let directory_to_create = PathBuf::from(directory);
        to_create.push_back(directory_to_create);
        parent = directory.parent();
    }

    while let Some(directory) = to_create.pop_back() {
        std::fs::create_dir(directory)?
    }

    Ok(())
}

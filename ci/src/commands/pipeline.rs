use std::path::PathBuf;

use crate::CiResult;

pub fn usage() -> String {
    format!("pipeline usage")
}

pub fn run(_args: Vec<String>, _target_dir: PathBuf) -> CiResult<()> {
    Err(std::io::Error::from(std::io::ErrorKind::BrokenPipe))?;
    Ok(())
}

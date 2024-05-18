use std::path::PathBuf;

use crate::CiResult;

pub fn usage() -> String {
    format!("archive usage")
}

pub fn run(
    _args: Vec<String>,
    _target_dir: PathBuf,
    _log_dir: PathBuf,
) -> CiResult<()> {
    crate::new_error_result!("archive")
}

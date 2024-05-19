use std::path::PathBuf;

use crate::CiResult;
use crate::ICommand;

pub struct Archive;

impl ICommand for Archive {
    fn usage() -> String {
        format!("archive")
    }

    fn run(_args: Vec<String>, _target_dir: PathBuf) -> CiResult<()> {
        crate::new_error_result!("archive")
    }
}

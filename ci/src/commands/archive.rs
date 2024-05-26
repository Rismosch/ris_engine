use std::path::PathBuf;

use crate::CiResult;
use crate::ICommand;

pub struct Archive;

impl ICommand for Archive {
    fn args() -> String {
        format!("[clean] [vendor] [compress]")
    }

    fn explanation() -> String {
        format!("Cleans, vendors and compresses the entire workspace.")
    }

    fn run(_args: Vec<String>, _target_dir: PathBuf) -> CiResult<()> {
        crate::new_error_result!("archive")
    }
}

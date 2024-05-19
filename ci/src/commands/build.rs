use std::path::PathBuf;

use crate::CiResult;
use crate::ICommand;

pub struct Build;
impl ICommand for Build {
    fn usage() -> String {
        format!("build       Generates build info and compiles the workspace as a release ready package")
    }

    fn run(_args: Vec<String>, _target_dir: PathBuf) -> CiResult<()> {
        crate::new_error_result!("not implemented")
    }
}

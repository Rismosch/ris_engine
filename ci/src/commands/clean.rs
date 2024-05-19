use std::path::PathBuf;

use crate::CiResult;
use crate::CiResultExtensions;
use crate::ICommand;

pub struct Clean;

impl ICommand for Clean {
    fn usage() -> String {
        format!("clean       Runs `cargo clean` and removes `./ci_out/`")
    }

    fn run(_args: Vec<String>, target_dir: PathBuf) -> CiResult<()> {
        crate::cmd::run("cargo clean")?;

        let parent = target_dir.parent().to_ci_result()?;
        let parent_name = parent.file_name().to_ci_result()?.to_str().to_ci_result()?;

        if parent_name == "ci_out" && parent.exists() {
            eprintln!("removing {:?}...", parent);
            std::fs::remove_dir_all(parent)?;
        }

        Ok(())
    }
}

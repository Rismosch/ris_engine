use std::path::PathBuf;

use crate::CiResult;
use crate::CiResultExtensions;
use crate::EmptyWrite;

pub fn usage() -> String {
    let name = env!("CARGO_PKG_NAME");
    format!("{} clean          runs `cargo clean` and removes `./ci_out/`", name, )
}

pub fn run(
    _args: Vec<String>, 
    target_dir: PathBuf, 
    _log_dir: PathBuf,
) -> CiResult<()> {
    crate::util::run_cmd::<EmptyWrite, EmptyWrite>("cargo clean", None)?;

    let parent = target_dir.parent().to_ci_result()?;
    let parent_name = parent.file_name()
        .to_ci_result()?
        .to_str()
        .to_ci_result()?;

    if parent_name == "ci_out" {
        std::fs::remove_dir_all(parent)?;
    }

    Ok(())
}

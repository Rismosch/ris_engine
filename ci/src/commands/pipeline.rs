use std::path::Path;
use std::path::PathBuf;

use crate::CiResult;
use crate::CmdStream;

pub fn usage() -> String {
    let name = env!("CARGO_PKG_NAME");
    format!(
        "{} pipeline       Runs various tests to determine, if the repo is in an acceptable state",
        name,
    )
}

pub fn run(_args: Vec<String>, _target_dir: PathBuf, log_dir: PathBuf) -> CiResult<()> {
    let now = chrono::Local::now();
    let result_dir_name = crate::util::sanitize_path(&now.to_rfc3339());
    let result_dir = log_dir.join(result_dir_name);

    let mut results = Vec::new();

    results.push(test(&result_dir, "cargo check")?);
    results.push(test(&result_dir, "cargo check -r")?);
    results.push(test(&result_dir, "cargo build")?);
    results.push(test(&result_dir, "cargo build -r")?);
    results.push(test(&result_dir, "cargo test")?);
    results.push(test(&result_dir, "cargo test -r")?);
    results.push(test(&result_dir, "cargo clippy -- -Dwarnings")?);
    results.push(test(&result_dir, "cargo clippy -r -- -Dwarnings")?);
    results.push(test(&result_dir, "cargo clippy --tests -- -Dwarnings")?);
    results.push(test(&result_dir, "cargo clippy -r --tests -- -Dwarnings")?);
    results.push(test(&result_dir, "cargo +nightly miri test")?);

    eprintln!("results:");
    for (cmd, success) in results {
        let success_str = match success {
            true => "OK:    ",
            false => "FAILED:",
        };

        println!("{} {}", success_str, cmd);
    }

    Ok(())
}

fn test<'a>(result_dir: &'a Path, cmd: &'a str) -> std::io::Result<(&'a str, bool)> {
    let stream = CmdStream::new(result_dir, cmd)?;

    let output = crate::util::run_cmd(cmd, stream);
    let success = match output {
        Ok(output) => match output.status.code() {
            Some(code) => code == 0,
            None => false,
        },
        Err(_) => false,
    };

    Ok((cmd, success))
}

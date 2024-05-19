use std::path::PathBuf;

use crate::CiResult;
use crate::ICommand;

pub struct Pipeline;

impl ICommand for Pipeline {
    fn usage() -> String {
        format!(
            "pipeline    Runs various tests, to determine if the repo is in an acceptable state"
        )
    }

    fn run(_args: Vec<String>, _target_dir: PathBuf) -> CiResult<()> {
        let mut results = Vec::new();

        results.push(test("cargo check")?);
        results.push(test("cargo check -r")?);
        results.push(test("cargo build")?);
        results.push(test("cargo build -r")?);
        results.push(test("cargo test")?);
        results.push(test("cargo test -r")?);
        results.push(test("cargo clippy -- -Dwarnings")?);
        results.push(test("cargo clippy -r -- -Dwarnings")?);
        results.push(test("cargo clippy --tests -- -Dwarnings")?);
        results.push(test("cargo clippy -r --tests -- -Dwarnings")?);
        results.push(test("cargo +nightly miri test")?);

        println!("results:");
        for (cmd, success) in results.iter() {
            let success_str = match success {
                true => "  OK:    ",
                false => "  FAILED:",
            };

            println!("{} {}", success_str, cmd);
        }

        if results.iter().all(|x| x.1) {
            Ok(())
        } else {
            crate::new_error_result!("pipeline failed")
        }
    }
}

fn test<'a>(cmd: &'a str) -> std::io::Result<(&'a str, bool)> {
    let exit_status = crate::cmd::run(cmd);
    let success = match exit_status {
        Ok(exit_status) => match exit_status.code() {
            Some(code) => code == 0,
            None => false,
        },
        Err(_) => false,
    };

    Ok((cmd, success))
}

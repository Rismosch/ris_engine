use std::path::PathBuf;

use crate::CiResult;
use crate::ICommand;

pub struct Pipeline;

impl ICommand for Pipeline {
    fn args() -> String {
        String::new()
    }

    fn explanation() -> String {
        format!("Runs various tests, to determine if the repo is in an acceptable state.")
    }

    fn run(_args: Vec<String>, _target_dir: PathBuf) -> CiResult<()> {
        let mut results = Vec::new();

        results.push(test("cargo check"));
        results.push(test("cargo check -r"));
        results.push(test("cargo build"));
        results.push(test("cargo build -r"));
        results.push(test("cargo test"));
        results.push(test("cargo test -r"));
        results.push(test("cargo clippy -- -Dwarnings"));
        results.push(test("cargo clippy -r -- -Dwarnings"));
        results.push(test("cargo clippy --tests -- -Dwarnings"));
        results.push(test("cargo clippy -r --tests -- -Dwarnings"));
        let cargo_nightly_miri_test = cargo_nightly("miri test")?;
        results.push(test(&cargo_nightly_miri_test));

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

fn test<'a>(cmd: &'a str) -> (&'a str, bool) {
    let exit_status = crate::cmd::run(cmd, None);
    let success = match exit_status {
        Ok(exit_status) => match exit_status.code() {
            Some(code) => code == 0,
            None => false,
        },
        Err(_) => false,
    };

    (cmd, success)
}

#[cfg(target_os = "windows")]
fn cargo_nightly(args: &str) -> CiResult<String> {
    let mut stdout = String::new();
    let exit_code = crate::cmd::run("where cargo", Some(&mut stdout))?;
    if !crate::cmd::has_exit_code(&exit_code, 0) {
        return crate::new_error_result!("failed to find cargo");
    }

    for line in stdout.lines() {
        if line.contains(".cargo") {
            return Ok(format!("{} +nightly {}", line, args));
        }
    }

    return crate::new_error_result!("failed to find nightly cargo");
}

#[cfg(not(target_os = "windows"))]
fn cargo_nightly(args: &str) -> CiResult<String> {
    Ok(format!("cargo +nightly {}", args))
}

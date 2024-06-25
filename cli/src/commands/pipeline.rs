use std::path::PathBuf;

use ris_error::RisResult;

use crate::ExplanationLevel;
use crate::ICommand;

pub struct Pipeline;

impl ICommand for Pipeline {
    fn args() -> String {
        String::new()
    }

    fn explanation(_level: ExplanationLevel) -> String {
        String::from("Runs various tests, to determine if the repo is in an acceptable state.")
    }

    fn run(_args: Vec<String>, _target_dir: PathBuf) -> RisResult<()> {
        let mut results = Vec::new();

        {
            let results = &mut results;
            test(results, "cargo check");
            test(results, "cargo check -r");
            test(results, "cargo build");
            test(results, "cargo build -r");
            test(results, "cargo test");
            test(results, "cargo test -r");
            test(results, &cargo_nightly("miri test")?);
            test(results, "cargo clippy -- -Dwarnings");
            test(results, "cargo clippy -r -- -Dwarnings");
            test(results, "cargo clippy --tests -- -Dwarnings");
            test(results, "cargo clippy -r --tests -- -Dwarnings");
            test(results, "cargo clippy -p cli -- -Dwarnings");
        }

        println!("done! finished running pipeline!");
        println!("results:");
        for (cmd, success) in results.iter() {
            let success_str = match success {
                true => "  OK:    ",
                false => "  FAILED:",
            };

            println!("{} {}", success_str, cmd);
        }

        if results.iter().all(|x| x.1) {
            println!("pipeline succeeded");
            Ok(())
        } else {
            println!("pipeline failed");
            ris_error::new_result!("pipeline failed")
        }
    }
}

fn test(results: &mut Vec<(String, bool)>, cmd: &str) {
    let exit_status = crate::cmd::run(cmd, None);
    let success = match exit_status {
        Ok(exit_status) => match exit_status.code() {
            Some(code) => code == 0,
            None => false,
        },
        Err(_) => false,
    };

    let result = (cmd.to_string(), success);
    results.push(result);
}

#[cfg(target_os = "windows")]
fn cargo_nightly(args: &str) -> RisResult<String> {
    let where_cargo = crate::cmd::run_where("cargo")?;

    for cargo in where_cargo {
        if cargo.contains(".cargo") {
            return Ok(format!("{} +nightly {}", cargo, args));
        }
    }

    ris_error::new_result!("failed to find nightly cargo")
}

#[cfg(not(target_os = "windows"))]
fn cargo_nightly(args: &str) -> RisResult<String> {
    Ok(format!("cargo +nightly {}", args))
}

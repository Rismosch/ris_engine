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
        let results = vec![
            test("cargo check"),
            test("cargo check -r"),
            test("cargo build"),
            test("cargo build -r"),
            test("cargo test"),
            test("cargo test -r"),
            test("cargo clippy -- -Dwarnings"),
            test("cargo clippy -r -- -Dwarnings"),
            test("cargo clippy --tests -- -Dwarnings"),
            test("cargo clippy -r --tests -- -Dwarnings"),
            test(&cargo_nightly("miri test")?),
            test("cargo clippy -p cli -- -Dwarnings"),
        ];

        println!("done! finished running all tests");
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

fn test(cmd: &str) -> (String, bool) {
    let exit_status = crate::cmd::run(cmd, None);
    let success = match exit_status {
        Ok(exit_status) => match exit_status.code() {
            Some(code) => code == 0,
            None => false,
        },
        Err(_) => false,
    };

    (cmd.to_string(), success)
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
fn cargo_nightly(args: &str) -> CiResult<String> {
    Ok(format!("cargo +nightly {}", args))
}

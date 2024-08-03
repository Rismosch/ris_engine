use std::path::PathBuf;

use ris_error::RisResult;

use crate::ExplanationLevel;
use crate::ICommand;

pub struct Pipeline;

const CHECK: &str = "check";
const BUILD: &str = "build";
const TEST: &str = "test";
const MIRI: &str = "miri";
const CLIPPY: &str = "clippy";
const ALL: &str = "all";
const NO_CHECK: &str = "no-check";
const NO_BUILD: &str = "no-build";
const NO_TEST: &str = "no-test";
const NO_MIRI: &str = "no-miri";
const NO_CLIPPY: &str = "no-clippy";

#[derive(PartialEq, Eq)]
enum TestResult {
    Ok,
    Failed,
    Skipped,
}

impl From<bool> for TestResult {
    fn from(value: bool) -> Self {
        if value {
            TestResult::Ok
        } else {
            TestResult::Failed
        }
    }
}

impl ICommand for Pipeline {
    fn args() -> String {
        format!(
            "[{}] [{}] [{}] [{}] [{}] [{}]",
            CHECK, BUILD, TEST, MIRI, CLIPPY, ALL,
        )
    }

    fn explanation(level: ExplanationLevel) -> String {
        match level {
            ExplanationLevel::Short => {
                String::from("Runs various tests, to determine if the repo is in an acceptable state.")
            },
            ExplanationLevel::Detailed => {
                let mut explanation = String::new();

                explanation.push_str(&format!("Runs various tests, to determine if the repo is in an acceptable state. Passing an arg runs the test of the according type. To exclude an arg, run `cli pipeline {} no-[arg]`\n", ALL));
                explanation.push('\n');
                explanation.push_str("args:\n");
                explanation.push('\n');
                explanation.push_str(&format!("{}\n", CHECK));
                explanation.push_str("Checks the repo for errors: https://doc.rust-lang.org/cargo/commands/cargo-check.html\n");
                explanation.push('\n');
                explanation.push_str(&format!("{}\n", BUILD));
                explanation.push_str("Builds the repo: https://doc.rust-lang.org/cargo/commands/cargo-build.html\n");
                explanation.push('\n');
                explanation.push_str(&format!("{}\n", TEST));
                explanation.push_str("Runs tests: https://doc.rust-lang.org/cargo/commands/cargo-test.html\n");
                explanation.push('\n');
                explanation.push_str(&format!("{}\n", MIRI));
                explanation.push_str("Runs tests with Miri: https://github.com/rust-lang/miri\n");
                explanation.push('\n');
                explanation.push_str(&format!("{}\n", CLIPPY));
                explanation.push_str("Linting: https://doc.rust-lang.org/stable/clippy/index.html\n");
                explanation.push('\n');
                explanation.push_str(&format!("{}\n", ALL));
                explanation.push_str("Runs all of the above\n");

                explanation
            },
        }
    }

    fn run(args: Vec<String>, _target_dir: PathBuf) -> RisResult<()> {
        if args.len() <= 2 {
            return crate::util::command_error(
                &format!("no args provided"),
                "pipeline",
                Self::args(),
                Self::explanation(ExplanationLevel::Detailed),
            );
        }

        let mut run_check = false;
        let mut run_build = false;
        let mut run_test = false;
        let mut run_miri = false;
        let mut run_clippy = false;

        for arg in &args[2..] {
            match arg.trim().to_lowercase().as_str() {
                CHECK => run_check = true,
                BUILD => run_build = true,
                TEST => run_test = true,
                MIRI => run_miri = true,
                CLIPPY => run_clippy = true,
                ALL => {
                    run_check = true;
                    run_build = true;
                    run_test = true;
                    run_miri = true;
                    run_clippy = true;
                },
                NO_CHECK => run_check = false,
                NO_BUILD => run_build = false,
                NO_TEST => run_test = false,
                NO_MIRI => run_miri = false,
                NO_CLIPPY => run_clippy = false,
                _ => {
                    return crate::util::command_error(
                        &format!("unkown arg: {}", arg),
                        "pipeline",
                        Self::args(),
                        Self::explanation(ExplanationLevel::Detailed),
                    );
                }
            }
        }

        let mut results = Vec::new();
        {
            let results = &mut results;
            test(results, run_check, "cargo check");
            test(results, run_check, "cargo check -r");
            test(results, run_check, "cargo check --no-default-features");
            test(results, run_check, "cargo check -r --no-default-features");
            test(results, run_build, "cargo build");
            test(results, run_build, "cargo build -r");
            test(results, run_build, "cargo build --no-default-features");
            test(results, run_build, "cargo build -r --no-default-features");
            test(results, run_test, "cargo test");
            test(results, run_test, "cargo test -r");
            test(results, run_test, "cargo test --no-default-features");
            test(results, run_test, "cargo test -r --no-default-features");
            test(results, run_miri, &cargo_nightly("miri test")?);
            test(results, run_clippy, "cargo clippy -- -Dwarnings");
            test(results, run_clippy, "cargo clippy -r -- -Dwarnings");
            test(results, run_clippy, "cargo clippy --no-default-features -- -Dwarnings");
            test(results, run_clippy, "cargo clippy --no-default-features -r -- -Dwarnings");
            test(results, run_clippy, "cargo clippy --tests -- -Dwarnings");
            test(results, run_clippy, "cargo clippy -r --tests -- -Dwarnings");
            test(results, run_clippy, "cargo clippy --tests --no-default-features -- -Dwarnings");
            test(results, run_clippy, "cargo clippy -r --tests --no-default-features -- -Dwarnings");
            test(results, run_clippy, "cargo clippy -p cli -- -Dwarnings");
            test(results, run_clippy, "cargo clippy -r -p cli -- -Dwarnings");
        }

        print_empty(5);

        println!("done! finished running pipeline!");
        println!("results:");
        for (cmd, result) in results.iter() {
            let success_str = match result {
                TestResult::Ok =>      "  ok:     ",
                TestResult::Failed =>  "  FAILED: ",
                TestResult::Skipped => "  skip:   ",
            };

            println!("{} {}", success_str, cmd);
        }

        if results.iter().all(|x| x.1 != TestResult::Failed) {
            println!("pipeline succeeded");
            print_empty(2);
            Ok(())
        } else {
            println!("pipeline failed");
            print_empty(2);
            ris_error::new_result!("pipeline failed")
        }
    }
}

fn test(
    results: &mut Vec<(String, TestResult)>,
    should_execute: bool,
    cmd: &str,
) {
    if !should_execute {
        let result = (cmd.to_string(), TestResult::Skipped);
        results.push(result);
        return;
    }

    let exit_status = crate::cmd::run(cmd, None);
    let success = match exit_status {
        Ok(exit_status) => match exit_status.code() {
            Some(code) => code == 0,
            None => false,
        },
        Err(_) => false,
    };

    let result = (cmd.to_string(), TestResult::from(success));
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

fn print_empty(lines: usize) {
    for _ in 0..lines {
        eprintln!()
    }
}

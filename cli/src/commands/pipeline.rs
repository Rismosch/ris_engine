use std::path::PathBuf;

use ris_error::RisResult;
use ris_io::fallback_file::FallbackFileAppend;

use crate::ExplanationLevel;
use crate::ICommand;

pub struct Pipeline;

const BUILD: &str = "build";
const TEST: &str = "test";
const MIRI: &str = "miri";
const CLIPPY: &str = "clippy";
const ALL: &str = "all";
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
        format!("[{}] [{}] [{}] [{}] [{}]", BUILD, TEST, MIRI, CLIPPY, ALL,)
    }

    fn explanation(level: ExplanationLevel) -> String {
        match level {
            ExplanationLevel::Short => String::from(
                "Runs various tests, to determine if the repo is in an acceptable state.",
            ),
            ExplanationLevel::Detailed => {
                let mut explanation = String::new();

                explanation.push_str(&format!("Runs various tests, to determine if the repo is in an acceptable state. Passing an arg runs the test of the according type. To exclude an arg, run `cli pipeline {} no-[arg]`\n", ALL));
                explanation.push('\n');
                explanation.push_str("args:\n");
                explanation.push('\n');
                explanation.push_str(&format!("{}\n", BUILD));
                explanation.push_str(
                    "Builds the repo: https://doc.rust-lang.org/cargo/commands/cargo-build.html\n",
                );
                explanation.push('\n');
                explanation.push_str(&format!("{}\n", TEST));
                explanation.push_str(
                    "Runs tests: https://doc.rust-lang.org/cargo/commands/cargo-test.html\n",
                );
                explanation.push('\n');
                explanation.push_str(&format!("{}\n", MIRI));
                explanation.push_str("Runs tests with Miri: https://github.com/rust-lang/miri\n");
                explanation.push('\n');
                explanation.push_str(&format!("{}\n", CLIPPY));
                explanation
                    .push_str("Linting: https://doc.rust-lang.org/stable/clippy/index.html\n");
                explanation.push('\n');
                explanation.push_str(&format!("{}\n", ALL));
                explanation.push_str("Runs all of the above\n");

                explanation
            }
        }
    }

    fn run(args: Vec<String>, target_dir: PathBuf) -> RisResult<()> {
        if args.len() <= 2 {
            return crate::util::command_error(
                "no args provided",
                "pipeline",
                Self::args(),
                Self::explanation(ExplanationLevel::Detailed),
            );
        }

        let mut fallback_file_append = FallbackFileAppend::new(&target_dir, ".txt", 10)?;
        let ff = &mut fallback_file_append;

        let mut run_build = false;
        let mut run_test = false;
        let mut run_miri = false;
        let mut run_clippy = false;

        for arg in &args[2..] {
            match arg.trim().to_lowercase().as_str() {
                BUILD => run_build = true,
                TEST => run_test = true,
                MIRI => run_miri = true,
                CLIPPY => run_clippy = true,
                ALL => {
                    run_build = true;
                    run_test = true;
                    run_miri = true;
                    run_clippy = true;
                }
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
            test(results, run_build, true, cargo("build"));
            test(results, run_build, true, cargo("build -r"));
            test(
                results,
                run_build,
                true,
                cargo("build -r --no-default-features"),
            );
            test(results, run_test, true, cargo("test"));
            test(results, run_test, true, cargo("test -r"));
            test(
                results,
                run_build,
                true,
                cargo("test -r --no-default-features"),
            );
            test(
                results,
                run_miri,
                false,
                cargo_nightly("miri test -r --no-default-features"),
            );
            test(results, run_clippy, false, cargo("clippy -- -Dwarnings"));
            test(results, run_clippy, false, cargo("clippy -r -- -Dwarnings"));
            test(
                results,
                run_build,
                false,
                cargo("clippy -r --no-default-features -- -Dwarnings"),
            );
            test(
                results,
                run_clippy,
                false,
                cargo("clippy --tests -- -Dwarnings"),
            );
            test(
                results,
                run_clippy,
                false,
                cargo("clippy -r --tests -- -Dwarnings"),
            );
            test(
                results,
                run_build,
                false,
                cargo("clippy -r --tests --no-default-features -- -Dwarnings"),
            );
            test(
                results,
                run_clippy,
                false,
                cargo("clippy -p cli -- -Dwarnings"),
            );
            test(
                results,
                run_clippy,
                false,
                cargo("clippy -r -p cli -- -Dwarnings"),
            );
            test(
                results,
                run_build,
                false,
                cargo("clippy -r -p cli --no-default-features -- -Dwarnings"),
            );
        }

        print_empty(ff, 2)?;

        print(ff, "done! finished running pipeline!")?;
        print(ff, "results:")?;
        for (cmd, result) in results.iter() {
            let success_str = match result {
                TestResult::Ok => "  ok:     ",
                TestResult::Failed => "  FAILED: ",
                TestResult::Skipped => "  skip:   ",
            };

            print(ff, format!("{} {}", success_str, cmd))?;
        }

        let result = if results.iter().all(|x| x.1 != TestResult::Failed) {
            print(ff, "pipeline succeeded")?;
            Ok(())
        } else {
            print(ff, "pipeline failed")?;
            ris_error::new_result!("pipeline failed")
        };

        print_empty(ff, 1)?;

        println!("results stored in \"{}\"", target_dir.display());

        print_empty(ff, 2)?;

        result
    }
}

fn test(
    results: &mut Vec<(String, TestResult)>,
    should_execute: bool,
    with_env: bool,
    cmd: Result<String, String>,
) {
    let env = ("RUSTFLAGS", "-D warnings");
    let env_str = "RUSTFLAGS=\"-D warnings\"";

    if !should_execute {
        let mut cmd = match cmd {
            Ok(cmd) => cmd,
            Err(cmd) => cmd,
        };

        if with_env {
            cmd = format!("{} {}", env_str, cmd);
        }

        let result = (cmd.to_string(), TestResult::Skipped);
        results.push(result);
        return;
    }

    let cmd = match cmd {
        Ok(cmd) => cmd,
        Err(cmd) => {
            eprintln!("error: failed to run command, because cargo could not be found");
            let result = (cmd, TestResult::Failed);
            results.push(result);
            return;
        }
    };

    let (exit_status, final_cmd) = if with_env {
        let exit_status = crate::cmd::run_with_envs(&cmd, [env]);
        let final_cmd = format!("{} {}", env_str, cmd);
        (exit_status, final_cmd)
    } else {
        let exit_status = crate::cmd::run(&cmd);
        let final_cmd = cmd.to_string();
        (exit_status, final_cmd)
    };

    let success = match exit_status {
        Ok(exit_status) => match exit_status.code() {
            Some(code) => code == 0,
            None => false,
        },
        Err(_) => false,
    };

    let result = (final_cmd, TestResult::from(success));
    results.push(result);
}

fn cargo(args: &str) -> Result<String, String> {
    Ok(format!("cargo {}", args))
}

#[cfg(target_os = "windows")]
fn cargo_nightly(args: &str) -> Result<String, String> {
    if let Ok(where_cargo) = crate::cmd::run_where("cargo") {
        for cargo in where_cargo {
            if cargo.contains(".cargo") {
                return Ok(format!("{} +nightly {}", cargo, args));
            }
        }
    }

    Err(format!("cargo +nightly {}", args))
}

#[cfg(not(target_os = "windows"))]
fn cargo_nightly(args: &str) -> Result<String, String> {
    Ok(format!("cargo +nightly {}", args))
}

fn print_empty(ff: &mut FallbackFileAppend, lines: usize) -> RisResult<()> {
    for _ in 0..lines {
        print(ff, "")?;
    }

    Ok(())
}

fn print(ff: &mut FallbackFileAppend, message: impl AsRef<str>) -> RisResult<()> {
    eprintln!("{}", message.as_ref());
    let stream = ff.current();
    ris_io::write(stream, message.as_ref().as_bytes())?;
    ris_io::write(stream, "\n".as_bytes())?;

    Ok(())
}

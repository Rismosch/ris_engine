use std::path::PathBuf;

use crate::CiResult;

pub fn usage() -> String {
    let name = env!("CARGO_PKG_NAME");
    format!(
        "{} pipeline       Runs various tests to determine, if the repo is in an acceptable state",
        name,
    )
}

pub fn run(_args: Vec<String>, _target_dir: PathBuf) -> CiResult<()> {
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
    results.push(test("cargo +nightly miri test"));

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

fn test(cmd: &str) -> (&str, bool) {
    let output = crate::util::run_cmd(cmd);
    let success = match output {
        Ok(output) => match output.status.code() {
            Some(code) => code == 0,
            None => false,
        },
        Err(_) => false,
    };

    (cmd, success)
}

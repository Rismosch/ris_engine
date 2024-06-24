use std::io::Read;

use ris_error::Extensions;
use ris_error::RisResult;

pub fn run(cmd: &str, stdout: Option<&mut String>) -> RisResult<std::process::ExitStatus> {
    eprintln!("running `{}`...", cmd);

    let splits = cmd.split(' ').map(|x| x.trim()).collect::<Vec<_>>();
    if splits.is_empty() {
        return ris_error::new_result!("cannot run empty cmd");
    }

    let mut command = std::process::Command::new(splits[0]);
    for arg in &splits[1..] {
        command.arg(arg);
    }

    if stdout.is_some() {
        command.stdout(std::process::Stdio::piped());
    }

    let mut process = command.spawn()?;
    if let Some(stdout_string) = stdout {
        let process_stdout = process.stdout.as_mut().unroll()?;
        process_stdout.read_to_string(stdout_string)?;
    }
    let exit_status = process.wait()?;

    match exit_status.code() {
        Some(code) => eprintln!("`{}` finished with exit code {}", cmd, code),
        None => eprintln!("`{}` finished with no exit code", cmd),
    }

    Ok(exit_status)
}

pub fn has_exit_code(exit_status: &std::process::ExitStatus, exit_code: i32) -> bool {
    if let Some(code) = exit_status.code() {
        if code == exit_code {
            return true;
        }
    }

    false
}

pub fn run_where(cmd: &str) -> RisResult<Vec<String>> {
    let cmd = format!("where {}", cmd);
    let mut stdout = String::new();

    run(&cmd, Some(&mut stdout))?;

    let result = stdout
        .split("\n")
        .map(|x| x.trim().to_string())
        .filter(|x| !x.is_empty())
        .collect::<Vec<_>>();

    Ok(result)
}

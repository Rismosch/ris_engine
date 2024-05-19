use crate::CiResult;

pub fn run(cmd: &str) -> CiResult<std::process::ExitStatus> {
    eprintln!("running `{}`...", cmd);

    let splits = cmd.split(' ').map(|x| x.trim()).collect::<Vec<_>>();
    if splits.is_empty() {
        return crate::new_error_result!("cannot run empty cmd");
    }

    let mut command = std::process::Command::new(splits[0]);
    for arg in &splits[1..] {
        command.arg(arg);
    }

    let mut process = command.spawn()?;
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

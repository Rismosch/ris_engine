use std::io::Read;

use ris_error::Extensions;
use ris_error::RisResult;

pub fn run(cmd: &str) -> RisResult<std::process::ExitStatus> {
    run_internal(cmd, None, None::<Vec<(&str, &str)>>)
}

pub fn run_with_stdout(cmd: &str, stdout: &mut String) -> RisResult<std::process::ExitStatus> {
    run_internal(cmd, Some(stdout), None::<Vec<(&str, &str)>>)
}

pub fn run_with_envs<I, K, V>(cmd: &str, envs: I) -> RisResult<std::process::ExitStatus>
where
    I: IntoIterator<Item = (K, V)>,
    K: AsRef<str>,
    V: AsRef<str>,
{
    run_internal(cmd, None, Some(envs))
}

fn run_internal<I, K, V>(
    cmd: &str,
    stdout: Option<&mut String>,
    envs: Option<I>,
) -> RisResult<std::process::ExitStatus>
where
    I: IntoIterator<Item = (K, V)>,
    K: AsRef<str>,
    V: AsRef<str>,
{
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

    let envs_string = if let Some(envs) = envs {
        let mut envs_string = String::new();

        for (key, value) in envs {
            let key_str = key.as_ref();
            let value_str = value.as_ref();
            command.env(key_str, value_str);

            let env_string = format!("{}=\"{}\"", key_str, value_str);
            envs_string.push_str(&env_string);
            envs_string.push(' ');
        }

        envs_string
    } else {
        String::new()
    };

    eprintln!("running `{}{}`...", envs_string, cmd);

    let mut process = command.spawn()?;
    if let Some(stdout_string) = stdout {
        let process_stdout = process.stdout.as_mut().into_ris_error()?;
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

    run_internal(&cmd, Some(&mut stdout), None::<Vec<(&str, &str)>>)?;

    let result = stdout
        .split('\n')
        .map(|x| x.trim().to_string())
        .filter(|x| !x.is_empty())
        .collect::<Vec<_>>();

    Ok(result)
}

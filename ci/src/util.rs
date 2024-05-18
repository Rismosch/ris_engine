use std::io::Read;
use std::io::Write;
use std::path::Path;

use crate::CmdStream;
use crate::CiResult;
use crate::CiResultExtensions;


pub fn run_cmd<Tstdout: Write, Tstderr: Write>(
    cmd: &str,
    mut stream: CmdStream<Tstdout, Tstderr>,
) -> CiResult<std::process::Output> {
    eprintln!("running `{}`...", cmd);

    let splits = cmd.split(' ').map(|x| x.trim()).collect::<Vec<_>>();
    if splits.is_empty() {
        return crate::new_error_result!("cannot run empty cmd");
    }

    let mut command = std::process::Command::new(splits[0]);
    for arg in &splits[1..] {
        command.arg(arg);
    }

    command.stdout(std::process::Stdio::piped());
    command.stderr(std::process::Stdio::piped());
    let mut process = command.spawn()?;

    let mut stdout_bytes = Vec::new();
    let mut stderr_bytes = Vec::new();
    process.stdout.take().to_ci_result()?.read_to_end(&mut stdout_bytes)?;
    process.stderr.take().to_ci_result()?.read_to_end(&mut stderr_bytes)?;
    stream.stdout.write(&stdout_bytes)?;
    stream.stderr.write(&stderr_bytes)?;

    let output = process.wait_with_output()?;

    match output.status.code() {
        Some(code) => eprintln!("`{}` finished with exit code {}", cmd, code),
        None => eprintln!("`{}` finished with no exit code", cmd),
    }

    Ok(output)
}

pub fn has_exit_code(output: &std::process::Output, exit_code: i32) -> bool {
    if let Some(code) = output.status.code() {
        if code == exit_code {
            return true;
        }
    }

    false
}

pub fn clean_or_create_dir(dir: &Path) -> std::io::Result<()> {
    if !dir.exists() {
        eprintln!("creating dir... {:?}", dir);
        std::fs::create_dir_all(dir)?;
    } else {
        eprintln!("cleaning dir... {:?}", dir);
        for entry in dir.read_dir()? {
            let entry = entry?;
            let path = entry.path();
            let metadata = entry.metadata()?;
            if metadata.is_file() {
                std::fs::remove_file(path)?;
            } else if metadata.is_dir() {
                std::fs::remove_dir_all(path)?;
            } else {
                return Err(std::io::Error::from(std::io::ErrorKind::Other));
            }
        }

        eprintln!("finished cleaning {:?}!", dir);
    }

    Ok(())
}

pub fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> std::io::Result<()> {
    std::fs::create_dir_all(&dst)?;
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let src = entry.path();
        let dst = dst.as_ref().join(entry.file_name());
        if entry.file_type()?.is_dir() {
            copy_dir_all(src, dst)?;
        } else {
            std::fs::copy(src, dst)?;
        }
    }

    Ok(())
}

pub fn sanitize_path(value: &str) -> String {
    const INVALID_CHARS: [char; 9] = [':', '*', '?', '"', '<', '>', '|', '\\', '/'];

    let mut value = String::from(value);
    for invalid_char in INVALID_CHARS {
        value = value.replace(invalid_char, "_");
    }

    value
}

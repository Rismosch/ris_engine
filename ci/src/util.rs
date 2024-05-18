use std::path::Path;

use crate::CiResult;

pub fn run_cmd(cmd: &str) -> CiResult<std::process::Output> {
    eprintln!("running `{}`...", cmd);

    let splits = cmd
        .split(' ')
        .map(|x| x.trim())
        .collect::<Vec<_>>();
    if splits.is_empty() {
        return crate::new_error_result!("cannot run empty cmd");
    }

    let mut command = std::process::Command::new(splits[0]);
    for arg in &splits[1..] {
        command.arg(arg);
    }

    let child = command.spawn()?;
    let output = child.wait_with_output()?;

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

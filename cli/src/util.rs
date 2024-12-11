use std::path::Path;
use std::path::PathBuf;

use ris_error::Extensions;
use ris_error::RisResult;

pub fn print_help_for_command(name: &str, args: String, explanation: String) {
    let cargo_pkg_name = env!("CARGO_PKG_NAME");
    eprintln!("usage: {} {} {}", cargo_pkg_name, name, args);
    eprintln!();
    eprintln!("{}", explanation);
}

pub fn command_error(
    message: &str,
    name: &str,
    args: String,
    explanation: String,
) -> RisResult<()> {
    eprintln!("{}", message);
    crate::util::print_help_for_command(name, args, explanation);
    ris_error::new_result!("{}", message)
}

pub fn get_root_dir() -> RisResult<PathBuf> {
    let output = std::process::Command::new(env!("CARGO"))
        .arg("locate-project")
        .arg("--workspace")
        .arg("--message-format=plain")
        .output()?
        .stdout;
    let cargo_path = Path::new(std::str::from_utf8(&output)?.trim());

    let root_dir = cargo_path.parent().into_ris_error()?.to_path_buf();

    Ok(root_dir)
}

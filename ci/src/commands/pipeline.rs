use crate::CiResult;

pub fn usage() -> &'static str {
    "pipeline usage"
}

pub fn run(_args: Vec<String>) -> CiResult<()> {
    Err(std::io::Error::from(std::io::ErrorKind::BrokenPipe))?;
    Ok(())
}

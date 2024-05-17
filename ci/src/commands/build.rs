use crate::CiResult;

pub fn usage() -> &'static str {
    "build usage"
}

pub fn run(_args: Vec<String>) -> CiResult<()> {
    crate::new_error_result!("build")
}

use std::io::Read;
use std::io::Seek;
use std::io::Write;

use ris_util::ris_error::RisError;

pub const IN_EXT: &str = "glsl";
pub const OUT_EXT: &str = "spirv";

pub fn import(
    input: &mut (impl Read + Seek),
    output: &mut (impl Write + Seek),
) -> Result<(), RisError> {
    let hello = "hello world";
    let test = hello.as_bytes();
    crate::util::write(output, test)?;

    Ok(())
}

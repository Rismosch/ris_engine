use std::fmt;

use sdl2::version::Version;

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct SdlInfo {
    version: Version,
    revision: String,
}

pub fn sdl_info() -> SdlInfo {
    SdlInfo {
        version: sdl2::version::version(),
        revision: sdl2::version::revision(),
    }
}

impl fmt::Display for SdlInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "SDL2 Info")?;
        writeln!(f, "Version:  {}", self.version)?;
        writeln!(f, "Revision: {}", self.revision)?;

        Ok(())
    }
}

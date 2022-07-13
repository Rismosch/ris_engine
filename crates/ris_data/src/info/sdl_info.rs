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
        write!(f, "SDL2\n")?;
        write!(f, "Version:  {}\n", self.version)?;
        write!(f, "Revision: {}\n", self.revision)?;

        Ok(())
    }
}

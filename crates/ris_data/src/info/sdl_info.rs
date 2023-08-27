use std::fmt;

use sdl2::version::Version;

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct SdlInfo {
    pub version: Version,
    pub revision: String,
}

impl SdlInfo {
    pub fn new() -> SdlInfo {
        SdlInfo {
            version: sdl2::version::version(),
            revision: sdl2::version::revision(),
        }
    }
}

impl Default for SdlInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for SdlInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "SDL2")?;
        writeln!(f, "Version:             {}", self.version)?;
        writeln!(f, "Revision:            {}", self.revision)?;

        Ok(())
    }
}

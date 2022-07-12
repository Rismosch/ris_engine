use std::fmt;

use super::{sdl_info::{SdlInfo, sdl_info}, cpu_info::{CpuInfo, cpu_info}};

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct RuntimeInfo {
    sdl: SdlInfo,
    cpu: CpuInfo,
}

pub fn runtime_info() -> RuntimeInfo {
    RuntimeInfo {
        sdl: sdl_info(),
        cpu: cpu_info(),
    }
}

impl fmt::Display for RuntimeInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "RUNTIME")?;
        writeln!(f, "{}", self.sdl)?;
        writeln!(f, "{}", self.cpu)?;

        Ok(())
    }
}

use std::fmt;

use super::{
    build_info::{build_info, BuildInfo},
    package_info::{package_info, PackageInfo},
    runtime_info::{runtime_info, RuntimeInfo},
    sdl_info::{sdl_info, SdlInfo},
};

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct AppInfo {
    build: BuildInfo,
    package: PackageInfo,
    runtime_info: RuntimeInfo,
    sdl: SdlInfo,
}

pub fn app_info() -> AppInfo {
    AppInfo {
        build: build_info(),
        package: package_info(),
        runtime_info: runtime_info(),
        sdl: sdl_info(),
    }
}

impl fmt::Display for AppInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self.package)?;
        writeln!(f, "{}", self.build)?;
        writeln!(f, "{}", self.runtime_info)?;
        writeln!(f, "{}", self.sdl)?;

        Ok(())
    }
}

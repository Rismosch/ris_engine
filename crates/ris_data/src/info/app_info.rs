use chrono::Local;
use std::fmt;

use super::{
    build_info::{build_info, BuildInfo},
    cpu_info::{cpu_info, CpuInfo},
    package_info::PackageInfo,
    sdl_info::{sdl_info, SdlInfo},
};

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct AppInfo {
    package: PackageInfo,
    build: BuildInfo,
    sdl: SdlInfo,
    cpu: CpuInfo,
}

pub fn app_info(package: PackageInfo) -> AppInfo {
    AppInfo {
        package,
        build: build_info(),
        sdl: sdl_info(),
        cpu: cpu_info(),
    }
}

impl fmt::Display for AppInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self.package)?;
        writeln!(f, "{}", self.build)?;
        writeln!(f, "{}", self.sdl)?;
        writeln!(f, "{}", self.cpu)?;
        write!(f, "Date\n{}", Local::now())?;

        Ok(())
    }
}

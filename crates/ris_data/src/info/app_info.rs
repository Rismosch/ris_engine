use chrono::Utc;
use std::fmt;

use super::{
    build_info::{build_info, BuildInfo},
    cpu_info::{cpu_info, CpuInfo},
    sdl_info::{sdl_info, SdlInfo},
    package_info::PackageInfo,
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
        package: package,
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
        writeln!(f, "Date\n{}", Utc::now())?;

        Ok(())
    }
}

use chrono::Local;
use std::fmt;

use super::{
    build_info::{build_info, BuildInfo},
    cpu_info::{cpu_info, CpuInfo},
    package_info::PackageInfo,
    sdl_info::{sdl_info, SdlInfo},
    file_info::{FileInfo, file_info},
};

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct AppInfo {
    pub package: PackageInfo,
    pub build: BuildInfo,
    pub file: FileInfo,
    pub sdl: SdlInfo,
    pub cpu: CpuInfo,
}

pub fn app_info(package: PackageInfo) -> AppInfo {
    let build = build_info();
    let file = file_info(&package);
    let sdl = sdl_info();
    let cpu = cpu_info();

    AppInfo {
        package,
        build,
        file,
        sdl,
        cpu,
    }
}

impl fmt::Display for AppInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self.package)?;
        writeln!(f, "{}", self.build)?;
        writeln!(f, "{}", self.file)?;
        writeln!(f, "{}", self.sdl)?;
        writeln!(f, "{}", self.cpu)?;

        Ok(())
    }
}

use chrono::Utc;
use std::fmt;

use super::{
    build_info::{build_info, BuildInfo},
    cpu_info::{cpu_info, CpuInfo},
    ipackage_info::IPackageInfo,
    sdl_info::{sdl_info, SdlInfo},
};

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct AppInfo<TPackageInfo: IPackageInfo> {
    package: TPackageInfo,
    build: BuildInfo,
    sdl: SdlInfo,
    cpu: CpuInfo,
}

pub fn app_info<TPackageInfo: IPackageInfo>() -> AppInfo<TPackageInfo> {
    AppInfo {
        package: TPackageInfo::new(),
        build: build_info(),
        sdl: sdl_info(),
        cpu: cpu_info(),
    }
}

impl<TPackageInfo: IPackageInfo + std::fmt::Display> fmt::Display for AppInfo<TPackageInfo> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self.package)?;
        writeln!(f, "{}", self.build)?;
        writeln!(f, "{}", self.sdl)?;
        writeln!(f, "{}", self.cpu)?;
        writeln!(f, "Date\n{}", Utc::now())?;

        Ok(())
    }
}

use chrono::Utc;
use std::fmt;

use super::{
    build_info::{build_info, BuildInfo},
    cpu_info::{CpuInfo, cpu_info},
    sdl_info::{SdlInfo, sdl_info},
    ipackage_info::IPackageInfo,
};

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct AppInfo<TPackageInfo: IPackageInfo> {
    package: TPackageInfo,
    build: BuildInfo,
    sdl: SdlInfo,
    cpu: CpuInfo,
}

pub fn app_info<TPackageInfo: IPackageInfo>() -> AppInfo<TPackageInfo>{
    AppInfo {
        package: TPackageInfo::new(),
        build: build_info(),
        sdl: sdl_info(),
        cpu: cpu_info(),
    }
}

impl<TPackageInfo: IPackageInfo + std::fmt::Display> fmt::Display for AppInfo<TPackageInfo> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\n", self.package)?;
        write!(f, "{}\n", self.build)?;
        write!(f, "{}\n", self.sdl)?;
        write!(f, "{}\n", self.cpu)?;
        write!(f, "Date: {}\n", Utc::now())?;

        Ok(())
    }
}

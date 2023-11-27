use std::fmt;

use super::args_info::ArgsInfo;
use super::build_info::BuildInfo;
use super::cpu_info::CpuInfo;
use super::file_info::FileInfo;
use super::package_info::PackageInfo;
use super::sdl_info::SdlInfo;

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct AppInfo {
    pub args: ArgsInfo,
    pub build: BuildInfo,
    pub cpu: CpuInfo,
    pub file: FileInfo,
    pub package: PackageInfo,
    pub sdl: SdlInfo,
}

impl AppInfo {
    pub fn new(
        args: ArgsInfo,
        build: BuildInfo,
        cpu: CpuInfo,
        file: FileInfo,
        package: PackageInfo,
        sdl: SdlInfo,
    ) -> Self {
        Self {
            args,
            build,
            cpu,
            file,
            package,
            sdl,
        }
    }
}

impl fmt::Display for AppInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self.package)?;
        writeln!(f, "{}", self.build)?;
        writeln!(f, "{}", self.file)?;
        writeln!(f, "{}", self.sdl)?;
        writeln!(f, "{}", self.cpu)?;
        writeln!(f, "{:?}", self.args)?;

        Ok(())
    }
}

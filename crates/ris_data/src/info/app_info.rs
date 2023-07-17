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
    pub package: PackageInfo,
    pub build: BuildInfo,
    pub file: FileInfo,
    pub sdl: SdlInfo,
    pub cpu: CpuInfo,
}

impl AppInfo {
    pub fn new(
        args: ArgsInfo,
        package: PackageInfo,
        build: BuildInfo,
        file: FileInfo,
        sdl: SdlInfo,
        cpu: CpuInfo,
    ) -> Self {
        Self {
            args,
            package,
            build,
            file,
            sdl,
            cpu,
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

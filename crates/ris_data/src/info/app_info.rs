use std::fmt;

use super::{
    super::cli_arguments::CliArguments,
    build_info::BuildInfo,
    cpu_info::CpuInfo,
    file_info::{file_info, FileInfo},
    package_info::PackageInfo,
    sdl_info::{sdl_info, SdlInfo},
};

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct AppInfo {
    pub args: CliArguments,
    pub package: PackageInfo,
    pub build: BuildInfo,
    pub file: FileInfo,
    pub sdl: SdlInfo,
    pub cpu: CpuInfo,
}

impl AppInfo {
    pub fn new(
        args: CliArguments,
        package: PackageInfo,
        build: BuildInfo,
        cpu_info: CpuInfo,
    ) -> AppInfo {
        let file = file_info(&package);
        let sdl = sdl_info();
        let cpu = cpu_info;

        AppInfo {
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

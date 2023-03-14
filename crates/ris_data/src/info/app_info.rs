use std::fmt;

use super::{
    build_info::{build_info, BuildInfo},
    cpu_info::{cpu_info, CpuInfo},
    file_info::{file_info, FileInfo},
    package_info::PackageInfo,
    sdl_info::{sdl_info, SdlInfo},
    super::cli_arguments::CliArguments,
};

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct AppInfo {
    pub package: PackageInfo,
    pub build: BuildInfo,
    pub file: FileInfo,
    pub sdl: SdlInfo,
    pub cpu: CpuInfo,
    pub args: CliArguments,
}

impl AppInfo{
    pub fn new(package: PackageInfo, cli_arguments: CliArguments) -> AppInfo {
        let build = build_info();
        let file = file_info(&package);
        let sdl = sdl_info();
        let cpu = cpu_info();
        let args = cli_arguments;

        AppInfo {
            package,
            build,
            file,
            sdl,
            cpu,
            args,
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

use std::fmt;

use super::{
    build_info::{build_info, BuildInfo},
    cpu_info::{cpu_info, CpuInfo},
    file_info::{file_info, FileInfo},
    package_info::PackageInfo,
    sdl_info::{sdl_info, SdlInfo},
};

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct AppInfo {
    pub package: PackageInfo,
    pub build: BuildInfo,
    pub file: FileInfo,
    pub sdl: SdlInfo,
    pub cpu: CpuInfo,
    pub args: Vec<String>,
}

pub fn app_info(package: PackageInfo) -> AppInfo {
    let build = build_info();
    let file = file_info(&package);
    let sdl = sdl_info();
    let cpu = cpu_info();
    let args = std::env::args().collect();

    AppInfo {
        package,
        build,
        file,
        sdl,
        cpu,
        args,
    }
}

impl fmt::Display for AppInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self.package)?;
        writeln!(f, "{}", self.build)?;
        writeln!(f, "{}", self.file)?;
        writeln!(f, "{}", self.sdl)?;
        writeln!(f, "{}", self.cpu)?;

        match self.args.len() {
            0 => writeln!(f, "no args")?,
            1 => writeln!(f, "1 arg")?,
            len => writeln!(f, "{} args", len)?,
        }

        for (i, arg) in self.args.iter().enumerate() {
            writeln!(f, "  [{}] -> {}", i, arg)?;
        }

        Ok(())
    }
}

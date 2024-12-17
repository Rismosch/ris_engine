use std::fmt;
use std::path::PathBuf;

use ris_error::RisResult;

use super::args_info::ArgsInfo;
use super::build_info::BuildInfo;
use super::cpu_info::CpuInfo;
use super::file_info::FileInfo;
use super::package_info::PackageInfo;
use super::sdl_info::SdlInfo;

#[derive(Default, Clone, Eq, PartialEq, Hash, Debug)]
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

    pub fn asset_path(&self) -> RisResult<PathBuf> {
        let mut path_buf = PathBuf::new();
        path_buf.push(&self.file.base_path);
        path_buf.push(String::from(&self.args.assets));
        if path_buf.exists() {
            Ok(path_buf)
        } else {
            // relative assets not found
            // search for assets absolute
            path_buf = PathBuf::new();
            path_buf.push(String::from(&self.args.assets));
            if path_buf.exists() {
                Ok(path_buf)
            } else {
                return ris_error::new_result!(
                    "failed to find assets \"{}\"",
                    &self.args.assets,
                );
            }
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

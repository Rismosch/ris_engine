use std::fmt;

use super::{
    build_info::{build_info, BuildInfo},
    package_info::{package_info, PackageInfo},
};

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct AppInfo {
    build: BuildInfo,
    package: PackageInfo,
}

pub fn app_info() -> AppInfo {
    AppInfo {
        build: build_info(),
        package: package_info(),
    }
}

impl fmt::Display for AppInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self.package)?;
        writeln!(f, "{}", self.build)?;

        Ok(())
    }
}

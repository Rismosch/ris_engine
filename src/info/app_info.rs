use std::fmt;

use super::{sdl_info::{SdlInfo, sdl_info}, build_info::{BuildInfo, build_info}, package_info::{PackageInfo, package_info}, runtime_info::{RuntimeInfo, runtime_info}};

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct AppInfo {
    build: BuildInfo,
    package: PackageInfo,
    runtime_info: RuntimeInfo,
    sdl: SdlInfo,
}

pub fn app_info() -> AppInfo {
    // let cargo = CargoInfo {
    //     manifest_dir: String::from(env!("CARGO_MANIFEST_DIR")),
    //     pkg_authors: String::from(env!("CARGO_PKG_AUTHORS")),
    //     pkg_homepage: String::from(env!("CARGO_PKG_HOMEPAGE")),
    //     pgk_name: String::from(env!("CARGO_PKG_NAME")),
    //     pkg_repository: String::from(env!("CARGO_PKG_REPOSITORY")),
    //     pkg_version: String::from(env!("CARGO_PKG_VERSION")),
    //     pkg_version_major: String::from(env!("CARGO_PKG_VERSION_MAJOR")),
    //     pkg_version_minor: String::from(env!("CARGO_PKG_VERSION_MINOR")),
    //     pkg_version_patch: String::from(env!("CARGO_PKG_VERSION_PATCH")),
    //     pkg_version_pre: String::from(env!("CARGO_PKG_VERSION_PRE")),
    // };

    AppInfo {
        build: build_info(),
        package: package_info(),
        runtime_info: runtime_info(),
        sdl: sdl_info()
    }
}

impl fmt::Display for AppInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        writeln!(f, "{}", self.package)?;
        writeln!(f, "{}", self.build)?;
        writeln!(f, "{}", self.runtime_info)?;
        writeln!(f, "{}", self.sdl)?;

        Ok(())
    }
}
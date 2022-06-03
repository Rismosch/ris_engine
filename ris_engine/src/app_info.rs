use std::fmt;

use sdl2::version::Version;

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct AppInfo {
    pub cargo: CargoInfo,
    pub sdl: SdlInfo,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct CargoInfo {
    pub manifest_dir: String,
    pub pkg_authors: String,
    pub pkg_homepage: String,
    pub pgk_name: String,
    pub pkg_repository: String,
    pub pkg_version: String,
    pub pkg_version_major: String,
    pub pkg_version_minor: String,
    pub pkg_version_patch: String,
    pub pkg_version_pre: String,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct SdlInfo {
    pub version: Version,
    pub revision: String,
    pub revision_number: i32,
}

pub fn app_info() -> AppInfo {

    let cargo = CargoInfo{
        manifest_dir: String::from(env!("CARGO_MANIFEST_DIR")),
        pkg_authors: String::from(env!("CARGO_PKG_AUTHORS")),
        pkg_homepage: String::from(env!("CARGO_PKG_HOMEPAGE")),
        pgk_name: String::from(env!("CARGO_PKG_NAME")),
        pkg_repository: String::from(env!("CARGO_PKG_REPOSITORY")),
        pkg_version: String::from(env!("CARGO_PKG_VERSION")),
        pkg_version_major: String::from(env!("CARGO_PKG_VERSION_MAJOR")),
        pkg_version_minor: String::from(env!("CARGO_PKG_VERSION_MINOR")),
        pkg_version_patch: String::from(env!("CARGO_PKG_VERSION_PATCH")),
        pkg_version_pre: String::from(env!("CARGO_PKG_VERSION_PRE")),
    };

    let version = sdl2::version::version();
    let revision = sdl2::version::revision();
    let revision_number = sdl2::version::revision_number();

    let sdl = SdlInfo{
        version,
        revision,
        revision_number,
    };

    AppInfo {
        cargo,
        sdl,
    }
}

impl fmt::Display for AppInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{} {}", self.cargo.pgk_name, self.cargo.pkg_version)?;
        writeln!(f, "by {}", self.cargo.pkg_authors)?;
        writeln!(f, "{}", self.cargo.pkg_repository)?;
        writeln!(f, "{}", self.cargo.pkg_homepage)?;
        writeln!(f, "\nSDL {}", self.sdl.version)?;
        writeln!(f, "    revision {}", self.sdl.revision)?;

        Ok(())
    }
}
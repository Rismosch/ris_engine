use std::fmt;

use ris_util::unwrap_or_throw;

use super::package_info::PackageInfo;

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct FileInfo {
    pub base_path: String,
    pub pref_path: String,
}

pub fn file_info(package_info: &PackageInfo) -> FileInfo {
    let base_path = unwrap_or_throw!(
        sdl2::filesystem::base_path(),
        "error while getting base path"
    );

    let pref_path = unwrap_or_throw!(
        sdl2::filesystem::pref_path(&package_info.author, &package_info.name),
        "error while getting pref path"
    );

    FileInfo {
        base_path,
        pref_path,
    }
}

impl fmt::Display for FileInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "File")?;
        writeln!(f, "Base Path:    {}", &self.base_path)?;
        writeln!(f, "Pref Path:    {}", &self.pref_path)?;

        Ok(())
    }
}

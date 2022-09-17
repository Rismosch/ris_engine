use std::fmt;

use super::package_info::PackageInfo;

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct FileInfo {
    pub base_path: String,
    pub pref_path: String,
}

pub fn file_info(package_info: &PackageInfo) -> FileInfo {
    let base_path = match sdl2::filesystem::base_path() {
        Ok(path) => path,
        Err(error) => panic!("error while getting base path: {}", error),
    };

    let pref_path = match sdl2::filesystem::pref_path(&package_info.author,&package_info.name) {
        Ok(path) => path,
        Err(error) => panic!("error while getting pref path: {}", error),
    };

    FileInfo {
        base_path,
        pref_path,
    }
}

impl fmt::Display for FileInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "File")?;
        writeln!(f, "base_path:    {}", &self.base_path)?;
        writeln!(f, "pref_path:    {}", &self.pref_path)?;

        Ok(())
    }
}
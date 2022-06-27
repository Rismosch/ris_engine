use std::fmt;

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct PackageInfo {
    name: String,
    version: String,
    author: String,
    website: String,
}

pub fn package_info() -> PackageInfo {
    PackageInfo {
        name: String::from(env!("CARGO_PKG_NAME")),
        version: String::from(env!("CARGO_PKG_VERSION")),
        author: String::from(env!("CARGO_PKG_AUTHORS")),
        website: String::from(env!("CARGO_PKG_HOMEPAGE")),
    }
}

impl fmt::Display for PackageInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{} v{}", self.name, self.version)?;
        writeln!(f, "author:  {}", self.author)?;
        writeln!(f, "website: {}", self.website)?;

        Ok(())
    }
}
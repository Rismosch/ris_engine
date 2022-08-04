use std::fmt;

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct PackageInfo {
    pub name: String,
    pub version: String,
    pub author: String,
    pub website: String,
}

// impl PackageInfo {
//     fn new() -> Self {
//         PackageInfo {
//             name: String::from(env!("CARGO_PKG_NAME")),
//             version: String::from(env!("CARGO_PKG_VERSION")),
//             author: String::from(env!("CARGO_PKG_AUTHORS")),
//             website: String::from(env!("CARGO_PKG_HOMEPAGE")),
//         }
//     }

//     fn name(&self) -> &str {
//         &self.name
//     }

//     fn version(&self) -> &str {
//         &self.version
//     }

//     fn author(&self) -> &str {
//         &self.author
//     }

//     fn website(&self) -> &str {
//         &self.website
//     }
// }

impl fmt::Display for PackageInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{} v{}", self.name, self.version)?;
        writeln!(f, "author:  {}", self.author)?;
        writeln!(f, "website: {}", self.website)?;

        Ok(())
    }
}

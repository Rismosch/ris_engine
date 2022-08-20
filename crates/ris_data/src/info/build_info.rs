// DO NOT PUSH CHANGES TO THIS FILE.
// THE CONTENTS OF THIS FILE WILL BE REPLACED BY `build.rs`.

use std::fmt;

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct BuildInfo {}

pub fn build_info() -> BuildInfo {
    BuildInfo {}
}

impl fmt::Display for BuildInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Build")?;
        writeln!(f, "`build.rs` was not run yet")?;

        Ok(())
    }
}

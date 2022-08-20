// DO NOT COMMIT CHANGES TO THIS FILE.
// THE CONTENTS OF THIS FILE WILL BE REPLACED BY `build.rs`.
// 
// I highly recommend you run the following git command:
// 
// git update-index --assume-unchanged crates/ris_data/src/info/build_info.rs
// 
// Doc: https://git-scm.com/docs/git-update-index#_using_assume_unchanged_bit

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

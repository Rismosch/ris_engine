// DO NOT COMMIT CHANGES TO THIS FILE.
// DO NOT MODIFY THIS FILE.
//
// THE CONTENTS OF THIS FILE ARE AUTOMATICALLY GENERATED BY THE BUILD SCRIPT.
//
// I highly recommend you run the following git command:
// git update-index --assume-unchanged crates/ris_data/src/info/build_info.rs
//
// Doc: https://git-scm.com/docs/git-update-index#_using_assume_unchanged_bit

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct BuildInfo;

pub fn build_info() -> BuildInfo {
    BuildInfo {}
}

impl std::fmt::Display for BuildInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "Build")?;
        writeln!(f, "build script was not run yet")?;

        Ok(())
    }
}

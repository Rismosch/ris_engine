#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct BuildInfo {
    git_repo: String,
    git_commit: String,
    git_branch: String,
    rustc_version: String,
    rustup_toolchain: String,
    build_profile: String,
    build_date: String,
}

impl BuildInfo {
    pub fn new() -> BuildInfo {
        //@@AUTO GENERATE START@@
        BuildInfo {
            git_repo: String::from(r"https://github.com/Rismosch/ris_engine.git"),
            git_commit: String::from(r"1981b3f864edfe2e2e63c562a2e556f9caba0323"),
            git_branch: String::from(r"main"),
            rustc_version: String::from(r"rustc 1.70.0 (90c541806 2023-05-31)"),
            rustup_toolchain: String::from(
                r"stable-x86_64-pc-windows-msvc (directory override for 'C:\Users\Rismosch\source\repos\ris_engine')",
            ),
            build_profile: profile(),
            build_date: String::from(r"2023-10-03T21:57:15.0807018+02:00"),
        }
        //@@AUTO GENERATE END@@
    }
}

impl Default for BuildInfo {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(debug_assertions)]
fn profile() -> String {
    String::from("debug")
}

#[cfg(not(debug_assertions))]
fn profile() -> String {
    String::from("release")
}

impl std::fmt::Display for BuildInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "Build")?;
        writeln!(f, "git repo:            {}", self.git_repo)?;
        writeln!(f, "git commit:          {}", self.git_commit)?;
        writeln!(f, "git branch:          {}", self.git_branch)?;
        writeln!(f, "compiler:            {}", self.rustc_version)?;
        writeln!(f, "toolchain:           {}", self.rustup_toolchain)?;
        writeln!(f, "profile:             {}", self.build_profile)?;
        writeln!(f, "build date:          {}", self.build_date)?;

        Ok(())
    }
}

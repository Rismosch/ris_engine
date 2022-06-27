use std::fmt;

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct RuntimeInfo {
}

pub fn runtime_info() -> RuntimeInfo {
    RuntimeInfo {}
}

impl fmt::Display for RuntimeInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Runtime Info")?;
        writeln!(f, "-")?;

        Ok(())
    }
}
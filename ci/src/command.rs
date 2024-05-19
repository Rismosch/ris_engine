use std::path::PathBuf;

use crate::CiResult;

pub trait ICommand {
    fn usage() -> String;
    fn run(args: Vec<String>, target_dir: PathBuf) -> CiResult<()>;
}

pub struct Command {
    pub name: String,
    pub run: Box<dyn Fn(Vec<String>, PathBuf) -> CiResult<()>>,
    pub usage: Box<dyn Fn() -> String>,
}

#[macro_export]
macro_rules! command {
    ($cmd:ident) => {{
        Command {
            name: stringify!($cmd).to_lowercase(),
            run: Box::new($cmd::run),
            usage: Box::new($cmd::usage),
        }
    }};
}

#[macro_export]
macro_rules! command_vec {
    ($($cmd:ident),+ $(,)*) => {{
        vec![$(command!($cmd)),+]
    }};
}

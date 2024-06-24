use std::path::PathBuf;

use ris_error::RisResult;

pub enum ExplanationLevel {
    Short,
    Detailed,
}

pub trait ICommand {
    fn args() -> String;
    fn explanation(level: ExplanationLevel) -> String;
    fn run(args: Vec<String>, target_dir: PathBuf) -> RisResult<()>;
}

pub struct Command {
    pub name: String,
    pub run: Box<dyn Fn(Vec<String>, PathBuf) -> RisResult<()>>,
    pub args: Box<dyn Fn() -> String>,
    pub explanation: Box<dyn Fn(ExplanationLevel) -> String>,
}

#[macro_export]
macro_rules! command {
    ($cmd:ident) => {{
        Command {
            name: stringify!($cmd).to_lowercase(),
            run: Box::new($cmd::run),
            args: Box::new($cmd::args),
            explanation: Box::new($cmd::explanation),
        }
    }};
}

#[macro_export]
macro_rules! command_vec {
    ($($cmd:ident),+ $(,)*) => {{
        vec![$(command!($cmd)),+]
    }};
}

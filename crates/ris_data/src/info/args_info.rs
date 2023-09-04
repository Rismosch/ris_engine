use std::env;

use ris_util::ris_error::RisError;
use ris_util::throw;

use crate::info::cpu_info::CpuInfo;

const NO_RESTART_ARG: &str = "--no-restart";
const WORKERS_ARG: &str = "--workers";
const ASSETS_ARG: &str = "--assets";

#[cfg(debug_assertions)]
fn create_with_default_values(
    raw_args: Vec<String>,
    executable_path: String,
    cpu_info: &CpuInfo,
) -> ArgsInfo {
    ArgsInfo {
        raw_args,
        executable_path,
        no_restart: false,
        workers: cpu_info.cpu_count,
        assets: String::from("assets"),
    }
}

#[cfg(not(debug_assertions))]
fn create_with_default_values(
    raw_args: Vec<String>,
    executable_path: String,
    cpu_info: &CpuInfo,
) -> ArgsInfo {
    ArgsInfo {
        raw_args,
        executable_path,
        no_restart: false,
        workers: cpu_info.cpu_count,
        assets: String::from("compiled.ris_assets"),
    }
}

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct ArgsInfo {
    pub raw_args: Vec<String>,
    pub executable_path: String,
    pub no_restart: bool,
    pub workers: i32,
    pub assets: String,
}

impl std::fmt::Debug for ArgsInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.raw_args.len() {
            0 => writeln!(f, "no commandline args:")?,
            1 => writeln!(f, "1 commandline arg:")?,
            len => writeln!(f, "{} commandline args:", len)?,
        }

        for (i, arg) in self.raw_args.iter().enumerate() {
            writeln!(f, "  [{}] -> {}", i, arg)?;
        }

        Ok(())
    }
}

impl std::fmt::Display for ArgsInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{")?;
        write!(f, "executable_path:\"{}\", ", self.executable_path)?;
        write!(f, "no_restart: {}", self.no_restart)?;
        write!(f, "assets: {}", self.assets)?;
        write!(f, "}}")?;
        Ok(())
    }
}

impl ArgsInfo {
    pub fn new(cpu_info: &CpuInfo) -> Result<Self, RisError> {
        let raw_args: Vec<String> = env::args().collect();
        let executable_path = String::from(&raw_args[0]);

        let mut result = create_with_default_values(raw_args, executable_path, cpu_info);

        let mut i = 1;
        let len = result.raw_args.len();
        loop {
            if i >= len {
                break;
            }

            let arg = &result.get_arg(i).to_lowercase()[..];

            match arg {
                NO_RESTART_ARG => result.no_restart = true,
                WORKERS_ARG => {
                    i += 1;
                    let second_arg = &result.get_arg(i);
                    match second_arg.parse::<i32>() {
                        Ok(value) => result.workers = value,
                        Err(error) => {
                            return ris_util::result_err!("could not parse workers: {}", error)
                        }
                    }
                }
                ASSETS_ARG => {
                    i += 1;
                    let second_arg = result.get_arg(i);
                    result.assets = String::from(second_arg);
                }
                _ => return ris_util::result_err!("unexpected argument: [{}] -> {}", i, arg),
            };

            i += 1;
        }

        Ok(result)
    }

    pub fn generate_raw_args(&self) -> Vec<String> {
        let mut result = Vec::new();

        result.push(self.executable_path.clone());

        if self.no_restart {
            result.push(String::from(NO_RESTART_ARG));
        }

        result.push(String::from(WORKERS_ARG));
        result.push(format!("{}", self.workers));

        result.push(String::from(ASSETS_ARG));
        result.push(String::from(&self.assets));

        result
    }

    fn get_arg(&self, index: usize) -> &str {
        match self.raw_args.get(index) {
            Some(arg) => arg,
            None => throw!(
                "index is out of bounds, index: {}, bounds: 0..{}",
                index,
                self.raw_args.len() - 1
            ),
        }
    }
}

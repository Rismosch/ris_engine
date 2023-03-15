use ris_util::throw;
use std::env;

use crate::info::cpu_info::CpuInfo;

const NO_RESTART_ARG: &str = "--no-restart";
const WORKERS_ARG: &str = "--workers";

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct ArgsInfo {
    pub raw_args: Vec<String>,
    pub executable_path: String,
    pub no_restart: bool,
    pub workers: i32,
}

impl ArgsInfo {
    pub fn new(cpu_info: &CpuInfo) -> Result<Self, String> {
        let raw_args: Vec<String> = env::args().collect();

        let executable_path = String::from(get_arg(&raw_args, 0));
        let mut no_restart = false;
        let mut workers = cpu_info.cpu_count;

        let mut i = 1;
        let len = raw_args.len();
        loop {
            if i >= len {
                break;
            }

            let arg = &get_arg(&raw_args, i).to_lowercase()[..];

            match arg {
                NO_RESTART_ARG => no_restart = true,
                WORKERS_ARG => {
                    i += 1;
                    let second_arg = get_arg(&raw_args, i);
                    match second_arg.parse::<i32>() {
                        Ok(value) => workers = value,
                        Err(error) => return Err(format!("could not parse workers: {}", error)),
                    }
                }
                _ => return Err(format!("unexpected argument: [{}] -> {}", i, arg)),
            };

            i += 1;
        }

        Ok(Self {
            raw_args,
            executable_path,
            no_restart,
            workers,
        })
    }

    pub fn generate_raw_args(&self) -> Vec<String> {
        let mut result = Vec::new();

        result.push(self.executable_path.clone());

        if self.no_restart {
            result.push(String::from(NO_RESTART_ARG));
        }

        result.push(String::from(WORKERS_ARG));
        result.push(format!("{}", self.workers));

        result
    }
}

fn get_arg(raw_args: &Vec<String>, index: usize) -> &str {
    match raw_args.get(index) {
        Some(arg) => arg,
        None => throw!(
            "index is out of bounds, index: {}, bounds: 0..{}",
            index,
            raw_args.len() - 1
        ),
    }
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
        write!(f, "}}")?;
        Ok(())
    }
}

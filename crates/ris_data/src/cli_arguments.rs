use ris_util::throw;
use std::env;

pub const NO_RESTART_ARG: &str = "--no-restart";

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct CliArguments {
    pub raw_args: Vec<String>,
    pub executable_path: String,
    pub no_restart: bool,
}

impl CliArguments {
    pub fn new() -> Result<Self, String> {
        let raw_args: Vec<String> = env::args().collect();

        let executable_path = String::from(get_arg(&raw_args, 0));
        let mut no_restart = false;

        let mut i = 1;
        let len = raw_args.len();
        loop {
            if i >= len {
                break;
            }

            let arg = &get_arg(&raw_args, i).to_lowercase()[..];

            match arg {
                NO_RESTART_ARG => no_restart = true,
                _ => return Err(format!("unexpected argument: [{}] -> {}", i, arg)),
            };

            i += 1;
        }

        Ok(Self {
            raw_args,
            executable_path,
            no_restart,
        })
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

impl std::fmt::Debug for CliArguments {
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

impl std::fmt::Display for CliArguments {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{")?;
        write!(f, "executable_path:\"{}\", ", self.executable_path)?;
        write!(f, "no_restart: {}", self.no_restart)?;
        write!(f, "}}")?;
        Ok(())
    }
}

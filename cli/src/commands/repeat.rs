use std::path::PathBuf;

use ris_error::RisResult;

use crate::ExplanationLevel;
use crate::ICommand;

pub struct Repeat;

impl ICommand for Repeat {
    fn args() -> String {
        String::from("-- <command> [args...]")
    }

    fn explanation(level: ExplanationLevel) -> String {
        match level {
            ExplanationLevel::Short => {
                String::from("Repeatedly runs a command.")
            }
            ExplanationLevel::Detailed => {
                let mut explanation = String::new();
                explanation.push_str("Repeatedly runs a command until it fails. If the command does not fail, this command will run forever. This means it may never end, and such should be cancelled with Ctrl+C.\n");
                explanation.push('\n');
                explanation.push_str("This can be very useful for stresstesting fuzzed tests.\n");
                explanation.push('\n');
                explanation
                    .push_str("The -- is necessary, to be able to pass args which start with -.\n");
                explanation.push('\n');
                explanation.push_str("example:\n");
                explanation.push_str("    cargo run -p cli repeat -- cargo test ris_math::color\n");
                explanation.push('\n');
                explanation.push_str("The command above repeatedly runs `cargo test ris_math::color` until it fails.\n");

                explanation
            }
        }
    }

    fn run(args: Vec<String>, _target_dir: PathBuf) -> RisResult<()> {
        let Some(divider) = args.iter().position(|x| x == "--") else {
            return crate::util::command_error(
                "missing --",
                "repeat",
                Self::args(),
                Self::explanation(ExplanationLevel::Detailed),
            );
        };

        let Some(command) = args.get(divider + 1) else {
            return crate::util::command_error(
                "no command provided",
                "repeat",
                Self::args(),
                Self::explanation(ExplanationLevel::Detailed),
            );
        };

        let args = &args[(divider + 2)..];

        let mut counter: u128 = 0;
        loop {
            counter = counter.wrapping_add(1);
            eprintln!("run {}...", counter);

            let mut command = std::process::Command::new(command);
            for arg in args {
                command.arg(arg);
            }

            let mut process = command.spawn()?;
            let exit_status = process.wait()?;

            match exit_status.code() {
                None => return ris_error::new_result!("process returned no exit code"),
                Some(code) => {
                    if code == 0 {
                        eprintln!("success!");
                        continue;
                    }

                    return ris_error::new_result!("process ended with exit code {}", code);
                }
            }
        }
    }
}

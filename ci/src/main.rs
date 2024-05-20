pub mod ci_error;
pub mod cmd;
pub mod command;
pub mod commands;
pub mod util;

use std::path::PathBuf;

pub use ci_error::CiResult;
pub use ci_error::CiResultExtensions;
pub use command::Command;
pub use command::ICommand;
pub use commands::archive::Archive;
pub use commands::build::Build;
pub use commands::doc::Doc;
pub use commands::pipeline::Pipeline;

fn main() -> Result<(), String> {
    let start = std::time::SystemTime::now();
    let raw_args = std::env::args().collect::<Vec<_>>();
    let commands = command_vec!(Archive, Build, Doc, Pipeline,);

    if raw_args.len() < 2 {
        print_help(commands);
        return Ok(());
    }

    let arg1 = &raw_args[1];
    if is_help_arg(arg1) {
        if raw_args.len() > 2 {
            let arg2 = &raw_args[2];
            let trimmed_arg = arg2.trim().to_lowercase();
            let command = commands.iter().find(|x| x.name == trimmed_arg);
            match command {
                Some(Command { usage, .. }) => eprintln!("usage: ci {}", usage()),
                None => {
                    eprintln!("unkown command: {}", arg2);
                    print_commands(commands);
                }
            }
        } else {
            print_help(commands);
        }
        return Ok(());
    }

    let trimmed_arg = arg1.trim().to_lowercase();
    let command = commands.iter().find(|x| x.name == trimmed_arg);

    match command {
        Some(Command { name, run, usage }) => {
            if raw_args.len() > 2 {
                let arg2 = &raw_args[2];
                if is_help_arg(arg2) {
                    eprintln!("usage: ci {}", usage());
                    return Ok(());
                }
            }

            let result = match get_target_dir(&raw_args[0], &name) {
                Ok(target_dir) => run(raw_args, target_dir),
                Err(e) => Err(e),
            };

            let end = std::time::SystemTime::now();
            if let Ok(duration) = end.duration_since(start) {
                eprintln!("finished in {:?}", duration);
            } else {
                eprintln!("failed to determine duration");
            }

            result.map_err(|e| e.to_string())
        }
        None => {
            eprintln!("unkown command: {}", arg1);
            print_commands(commands);
            Ok(())
        }
    }
}

fn print_help(to_print: Vec<Command>) {
    let name = env!("CARGO_PKG_NAME");
    eprintln!("usage: {} <command> [help]", name);
    print_commands(to_print);
}

fn print_commands(to_print: Vec<Command>) {
    eprintln!("commands:");
    for command in to_print {
        eprintln!("    {}", (command.usage)());
    }
}

fn is_help_arg(arg: &str) -> bool {
    let arg = arg.trim().to_lowercase();

    arg == "h"
        || arg == "-h"
        || arg == "--h"
        || arg == "help"
        || arg == "-help"
        || arg == "--help"
        || arg == "man"
        || arg == "-man"
        || arg == "--man"
        || arg == "manual"
        || arg == "-manual"
        || arg == "--manual"
}

fn get_target_dir(program: &str, command: &str) -> CiResult<PathBuf> {
    let parent = match crate::util::get_root_dir() {
        Ok(root_dir) => root_dir,
        Err(_) => PathBuf::from(program)
            .parent()
            .to_ci_result()?
            .to_path_buf(),
    };

    let target_dir = parent.join("ci_out").join(command);

    Ok(target_dir)
}

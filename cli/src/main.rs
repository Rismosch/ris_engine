pub mod cmd;
pub mod command;
pub mod commands;
pub mod util;

use std::path::PathBuf;

use ris_error::Extensions;
use ris_error::RisResult;

pub use command::Command;
pub use command::ExplanationLevel;
pub use command::ICommand;
pub use commands::archive::Archive;
pub use commands::asset::Asset;
pub use commands::asset::AssetCommand;
pub use commands::build::Build;
pub use commands::doc::Doc;
pub use commands::pipeline::Pipeline;
pub use commands::profiler_html::ProfilerHtml;
pub use commands::repeat::Repeat;

fn main() -> Result<(), String> {
    let start = std::time::SystemTime::now();

    unsafe {
        ris_error::error::PRINT_WARNING_ON_BACKTRACE = false;
        ris_file::util::TRACE = true;
    }

    let mut raw_args = std::env::args().collect::<Vec<_>>();
    let verbose_position = raw_args.iter().position(|x| is_verbose_arg(x));
    let verbose = if let Some(verbose_position) = verbose_position {
        raw_args.remove(verbose_position);
        true
    } else {
        false
    };

    let commands = command_vec!(Archive, Asset, Build, Doc, Pipeline, ProfilerHtml, Repeat,);

    // check if no arguments provided
    if raw_args.len() < 2 {
        print_help(commands);
        return Ok(());
    }

    // check if `help` is the first command
    let arg1 = &raw_args[1];
    if is_help_arg(arg1) {
        if raw_args.len() > 2 {
            let arg2 = &raw_args[2];
            let trimmed_arg = arg2.trim().to_lowercase();
            let command = commands.iter().find(|x| x.name == trimmed_arg);
            match command {
                Some(Command {
                    name,
                    args,
                    explanation,
                    ..
                }) => {
                    crate::util::print_help_for_command(
                        name,
                        args(),
                        explanation(ExplanationLevel::Detailed),
                    );
                }
                None => {
                    eprintln!("unkown command: {}", arg2);
                    print_help(commands);
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
        Some(Command {
            name,
            run,
            args,
            explanation,
        }) => {
            // check if `help` is the second command
            if raw_args.len() > 2 {
                let arg2 = &raw_args[2];
                if is_help_arg(arg2) {
                    crate::util::print_help_for_command(
                        name,
                        args(),
                        explanation(ExplanationLevel::Detailed),
                    );
                    return Ok(());
                }
            }

            // run command
            let result = match get_target_dir(&raw_args[0], name) {
                Ok(target_dir) => run(raw_args, target_dir),
                Err(e) => Err(e),
            };

            let end = std::time::SystemTime::now();
            if let Ok(duration) = end.duration_since(start) {
                eprintln!("finished in {:?}", duration);
            } else {
                eprintln!("failed to determine duration");
            }

            result.map_err(|e| {
                if verbose {
                    eprintln!("    {:?}", e);
                } else {
                    eprintln!("command failed. pass -v for more info");
                }

                if let Some(message) = e.message {
                    message
                } else if let Some(source) = e.source {
                    source.to_string()
                } else {
                    String::from("error contained no information on what caused it")
                }
            })
        }
        None => {
            eprintln!("unkown command: {}", arg1);
            print_help(commands);
            Ok(())
        }
    }
}

fn print_help(to_print: Vec<Command>) {
    let mut max_name_len = 0;
    let mut max_args_len = 0;
    let mut max_explanation_len = 0;

    for Command {
        name,
        args,
        explanation,
        ..
    } in to_print.iter()
    {
        max_name_len = usize::max(max_name_len, name.len());
        max_args_len = usize::max(max_args_len, args().len());
        max_explanation_len = usize::max(
            max_explanation_len,
            explanation(ExplanationLevel::Short).len(),
        );
    }

    let cargo_pkg_name = env!("CARGO_PKG_NAME");
    eprintln!("usage: {} [help] <command>", cargo_pkg_name);
    eprintln!("commands:");
    for Command {
        name, explanation, ..
    } in to_print
    {
        let mut name = name;
        while name.len() < max_name_len {
            name.push(' ');
        }

        let mut explanation = explanation(ExplanationLevel::Short);
        while explanation.len() < max_explanation_len {
            explanation.push(' ');
        }

        let mut line = format!("    {}   ", name);
        for word in explanation.split(' ') {
            if line.len() + word.len() > 50 {
                eprintln!("{}", line);
                let offset = 7 + max_name_len;
                line = String::new();
                for _ in 0..offset {
                    line.push(' ');
                }
            }

            line.push_str(&format!(" {}", word));
        }

        if !line.trim().is_empty() {
            eprintln!("{}", line);
        }
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

fn is_verbose_arg(arg: &str) -> bool {
    let arg = arg.trim().to_lowercase();

    arg == "v"
        || arg == "-v"
        || arg == "--v"
        || arg == "verbose"
        || arg == "-verbose"
        || arg == "--verbose"
}

fn get_target_dir(program: &str, command: &str) -> RisResult<PathBuf> {
    let parent = match crate::util::get_root_dir() {
        Ok(root_dir) => root_dir,
        Err(_) => PathBuf::from(program).parent().unroll()?.to_path_buf(),
    };

    let cargo_pkg_name = env!("CARGO_PKG_NAME");
    let target_dir_name = format!("{}_out", cargo_pkg_name);
    let target_dir = parent.join(target_dir_name).join(command);

    Ok(target_dir)
}

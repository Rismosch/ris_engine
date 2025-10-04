mod commands;
mod cmd;
mod util;

use std::path::PathBuf;

use ris_error::prelude::*;

use commands::ExplanationLevel;
use commands::ICommand;

const OUT_DIR_NAME: &str = "cli_out";

// expected args:
// arg[0]:   [path to executable]
// arg[1]:   "cli"
// arg[2/3]: command/<help>
// arg[3..]: [args for the command]
pub fn run<T: AsRef<str>>(args: impl IntoIterator<Item = T>) -> RisResult<()> {
    let start = std::time::SystemTime::now();

    unsafe {
        ris_error::error::PRINT_WARNING_ON_BACKTRACE = false;
        ris_io::util::TRACE = true;
    }

    let mut args = args.into_iter()
        .map(|x|x.as_ref().to_string())
        .collect::<Vec<_>>();
    let verbose_position = args.iter().position(|x| is_verbose_arg(x));
    let verbose = if let Some(verbose_position) = verbose_position {
        args.remove(verbose_position);
        true
    } else {
        false
    };

    let commands: Vec<Box<dyn ICommand>> = vec![
        Box::new(commands::asset::Asset),
        Box::new(commands::build::Build),
        Box::new(commands::doc::Doc),
        Box::new(commands::god_asset::GodAsset),
        Box::new(commands::pipeline::Pipeline),
        Box::new(commands::profiler_html::ProfilerHtml),
    ];

    // check if no arguments provided
    if args.len() < 3 {
        print_help(&commands);
        return Ok(());
    }

    // check if `help` is the first command
    let arg2 = &args[2];
    if is_help_arg(arg2) {
        if args.len() > 3 {
            let arg3 = &args[3];
            let trimmed_arg = arg3.trim().to_lowercase();
            let command = commands.iter().find(|x| x.name() == trimmed_arg);
            match command {
                Some(command) => {
                    util::print_help_for_command(
                        command.as_ref(),
                        ExplanationLevel::Detailed,
                    );
                }
                None => {
                    eprintln!("unkown command: {}", arg2);
                    print_help(&commands);
                }
            }
        } else {
            print_help(&commands);
        }
        return Ok(());
    }

    let trimmed_arg = arg2.trim().to_lowercase();
    let command = commands.iter().find(|x| x.name() == trimmed_arg);

    match command {
        Some(command) => {
            // check if `help` is the second command
            if args.len() > 3 {
                let arg3 = &args[3];
                if is_help_arg(arg3) {
                    util::print_help_for_command(
                        command.as_ref(),
                        ExplanationLevel::Detailed,
                    );
                    return Ok(());
                }
            }

            // run command
            let target_dir = get_target_dir(&args[0], command.name());
            let result = match target_dir {
                Ok(target_dir) => command.run(args, &target_dir),
                Err(e) => Err(e),
            };

            let end = std::time::SystemTime::now();
            if let Ok(duration) = end.duration_since(start) {
                eprintln!("finished in {:?}", duration);
            } else {
                eprintln!("failed to determine duration");
            }

            result.map_err(|mut e| {
                if !verbose {
                    e.backtrace.take();
                    eprintln!("command failed. pass -v for more info");
                }

                e
            })
        }
        None => {
            eprintln!("unkown command: {}", arg2);
            print_help(&commands);
            Ok(())
        }
    }
}

fn print_help(commands: &[Box<dyn ICommand>]) {
    let mut max_name_len = 0;
    let mut max_args_len = 0;
    let mut max_explanation_len = 0;

    for command in commands.iter() {
        max_name_len = usize::max(max_name_len, command.name().len());
        max_args_len = usize::max(max_args_len, command.args().len());
        max_explanation_len = usize::max(
            max_explanation_len,
            command.explanation(ExplanationLevel::Short).len(),
        );
    }

    let cargo_pkg_name = env!("CARGO_PKG_NAME");
    eprintln!(
        "usage: {} {} [help] <command>",
        super::CLI,
        cargo_pkg_name,
    );
    eprintln!("commands:");
    for command in commands {
        let mut name = command.name();
        while name.len() < max_name_len {
            name.push(' ');
        }

        let mut explanation = command.explanation(ExplanationLevel::Short);
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

fn is_verbose_arg(arg: &str) -> bool {
    let arg = arg.trim().to_lowercase();

    arg == "v"
        || arg == "-v"
        || arg == "--v"
        || arg == "verbose"
        || arg == "-verbose"
        || arg == "--verbose"
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

fn get_target_dir(program: impl AsRef<str>, command_name: impl AsRef<str>) -> RisResult<PathBuf> {
    let program = program.as_ref();
    let command_name = command_name.as_ref();

    let root_dir = util::get_root_dir();
    let parent = match root_dir {
        Ok(root_dir) => root_dir,
        Err(_) => PathBuf::from(program)
            .parent()
            .into_ris_error()?
            .to_path_buf(),
    };

    let target_dir = parent.join(OUT_DIR_NAME).join(command_name);

    Ok(target_dir)
}

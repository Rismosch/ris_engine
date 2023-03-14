use ris_core::engine::Engine;
use ris_data::info::app_info::AppInfo;
use ris_data::info::{package_info::PackageInfo};
use ris_data::cli_arguments::{CliArguments, NO_RESTART_ARG};
use ris_data::package_info;
use ris_log::{
    log::{self, Appenders, LogGuard},
    log_level::LogLevel,
};
use ris_util::{unwrap_or_throw, throw};

pub const RESTART_CODE: i32 = 42;

fn main() -> Result<(), String> {
    let cli_arguments_result = CliArguments::new();
    let cli_arguments = match cli_arguments_result {
        Ok(cli_arguments) => cli_arguments,
        Err(error) => return Err(format!("error while parsing cli args: {}", error)),
    };

    if cli_arguments.no_restart {
        run(cli_arguments)
    } else {
        wrap(cli_arguments);
        Ok(())
    }
}

fn wrap(cli_arguments: CliArguments)
{
    loop {
        let mut command = std::process::Command::new(&cli_arguments.executable_path);
        
        command.arg(NO_RESTART_ARG);

        /*for arg in std::env::args().into_iter().skip(1) {
            command.arg(arg);
        }*/

        let child = unwrap_or_throw!(command.spawn(), "child could not be spawned");
        let output = unwrap_or_throw!(child.wait_with_output(), "child could not be awaited");

        let exit_code = if let Some(code) = output.status.code() {
            println!("process finished with code {}", code);

            if code == RESTART_CODE {
                println!("restarting...");
                continue;
            } else {
                Some(code)
            }
        } else {
            println!("process finished with no code");
            None
        };

        if output.status.success() {
            return;
        } else {
            let output_bytes = output.stderr;
            let output_string = String::from_utf8(output_bytes);

            match output_string {
                Ok(to_print) => eprintln!("{}", to_print),
                Err(error) => throw!("error while formatting output.stderr: {}", error),
            }

            match exit_code {
                Some(code) => std::process::exit(code),
                None => return,
            }
        }
    }
}

fn run(cli_arguments: CliArguments) -> Result<(), String>
{
    let package_info = package_info!();
    let app_info = AppInfo::new(package_info, cli_arguments);
    let log_guard = init_log(&app_info);

    let mut engine = Engine::new(app_info)?;
    let result = engine.run();

    if let Err(error) = result {
        ris_log::fatal!("error while running engine: \"{}\"", error);
    }

    drop(log_guard);

    let exit_code = if engine.wants_to_restart {
        std::process::exit(RESTART_CODE);
    } else {
        0
    };

    Ok(())
}

#[cfg(debug_assertions)]
fn init_log(app_info: &AppInfo) -> LogGuard {
    use ris_core::appenders::{console_appender::ConsoleAppender, file_appender::FileAppender};

    let appenders: Appenders = vec![ConsoleAppender::new(), FileAppender::new(app_info)];
    log::init(LogLevel::Trace, appenders)
}

#[cfg(not(debug_assertions))]
fn init_log(app_info: &AppInfo) -> LogGuard {
    use ris_core::appenders::file_appender::FileAppender;

    let appenders: Appenders = vec![FileAppender::new(app_info)];
    log::init(LogLevel::Info, appenders)
}

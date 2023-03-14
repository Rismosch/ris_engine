use ris_core::engine::Engine;
use ris_data::info::app_info::AppInfo;
use ris_data::info::{app_info::app_info, package_info::PackageInfo};
use ris_data::cli_arguments::CliArguments;
use ris_data::package_info;
use ris_log::{
    log::{self, Appenders, LogGuard},
    log_level::LogLevel,
};

fn main() -> Result<(), String> {

    let cli_arguments = CliArguments::new();
    println!("cli_arguments: {}", cli_arguments.unwrap());


    loop {

        println!("hello world");
        break;
        /*
        let mut command = std::process::Command::new("ris_engine.exe");

        for arg in std::env::args().into_iter().skip(1) {
            command.arg(arg);
        }

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
        }*/
    }
    return Ok(());
    /*let app_info = app_info(package_info!());
    let log_guard = init_log(&app_info);

    let result = Engine::new(app_info)?.run();

    match result {
        Ok(exit_code) => ris_log::info!("exit code {}", exit_code),
        Err(ref error) => ris_log::fatal!("exit error \"{}\"", error),
    }

    drop(log_guard);

    let exit_code = result?;
    std::process::exit(exit_code);*/
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

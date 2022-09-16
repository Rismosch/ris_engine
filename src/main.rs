use ris_core::engine::Engine;
use ris_data::info::package_info::PackageInfo;
use ris_data::package_info;
use ris_log::{
    appenders::console_appender::ConsoleAppender,
    log::{self, Appenders},
    log_level::LogLevel,
};

fn main() -> Result<(), String> {
    let appenders: Appenders = vec![ConsoleAppender::new()];
    let log_guard = log::init(LogLevel::Trace, appenders);

    let package_info = package_info!();
    let result = Engine::new(package_info)?.run();

    match result {
        Ok(exit_code) => ris_log::info!("exit code {}", exit_code),
        Err(ref error) => ris_log::fatal!("exit error \"{}\"", error),
    }

    drop(log_guard);

    let exit_code = result?;
    std::process::exit(exit_code);
}

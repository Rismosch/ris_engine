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
    let exit_code = Engine::new(package_info)?.run()?;

    ris_log::info!("exit code: {}", exit_code);

    drop(log_guard);

    std::process::exit(exit_code);
}

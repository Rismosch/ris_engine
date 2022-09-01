use ris_core::engine::Engine;
use ris_data::info::package_info::PackageInfo;
use ris_data::package_info;
use ris_log::{
    appenders::{console_appender::ConsoleAppender, i_appender::IAppender},
    log,
    log_level::LogLevel,
};

fn main() -> Result<(), String> {
    let appenders: Vec<Box<(dyn IAppender + 'static)>> = vec![ConsoleAppender::new()];
    let log_guard = log::init(LogLevel::Trace, appenders, false);

    let package_info = package_info!();
    Engine::new(package_info)?.run()?;

    drop(log_guard);

    Ok(())
}

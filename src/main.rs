use ris_core::engine::Engine;
use ris_data::info::package_info::PackageInfo;
use ris_data::package_info;
use ris_log::{log, log_level::LogLevel, appenders::{console_appender::ConsoleAppender, i_appender::IAppender}};

fn main() -> Result<(), String> {
    let appenders: Vec<Box<(dyn IAppender + 'static)>> = vec![ConsoleAppender::new()];
    log::init(LogLevel::Trace, appenders);

    let package_info = package_info!();
    Engine::new(package_info)?.run()?;

    log::drop();

    Ok(())
}

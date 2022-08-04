use ris_core::engine::Engine;
use ris_data::info::package_info::{PackageInfo};
use ris_data::package_info;
use ris_log::console_appender::ConsoleAppender;
use ris_log::{log,log_level::LogLevel};

fn main() -> Result<(), String> {
    log::init(LogLevel::Trace);
    log::register_appender(ConsoleAppender {});

    let package_info = package_info!();

    Engine::new(package_info)?.run()?;

    log::drop();

    Ok(())
}

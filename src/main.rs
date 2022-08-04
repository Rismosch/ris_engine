use ris_core::engine::Engine;
use ris_data::info::package_info::{PackageInfo};
use ris_data::package_info;
use ris_log::console_appender::ConsoleAppender;
use ris_log::i_appender::IAppender;
use ris_log::{log,log_level::LogLevel};

fn main() -> Result<(), String> {

    let appenders: Vec<Box<(dyn IAppender + 'static)>> = vec![ConsoleAppender::new()];
    log::init(LogLevel::Trace,appenders);

    let package_info = package_info!();
    Engine::new(package_info)?.run()?;

    ris_log::debug!("hello world");
    ris_log::debug!("{}", 42);

    log::drop();

    Ok(())
}

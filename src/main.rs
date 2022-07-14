pub mod package_info;

use package_info::PackageInfo;
use ris_core::engine::Engine;
use ris_log::console_appender::ConsoleAppender;
use ris_log::{log,log_level::LogLevel};

fn main() -> Result<(), String> {
    log::init(LogLevel::Trace, false);
    log::register_appender(ConsoleAppender {});

    Engine::<PackageInfo>::new()?.run()?;

    log::drop();

    Ok(())
}

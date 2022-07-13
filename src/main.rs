pub mod package_info;

use package_info::PackageInfo;
use ris_core::engine::Engine;
use ris_log::console_appender::ConsoleAppender;
use ris_log::log_level::LogLevel;

fn main() -> Result<(), String> {
    ris_log::log::init(LogLevel::Trace, false);
    ris_log::log::register_appender(ConsoleAppender{});

    let mut engine = Engine::<PackageInfo>::new()?;

    engine.run()
}

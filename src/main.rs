use ris_core::engine::Engine;
use ris_data::info::package_info::PackageInfo;
use ris_log::console_appender::ConsoleAppender;
use ris_log::{log,log_level::LogLevel};

fn main() -> Result<(), String> {
    log::init(LogLevel::Trace);
    log::register_appender(ConsoleAppender {});

    let package_info = PackageInfo{
        name: String::from("dieter flachmann"),
        version: String::from("dieter flachmann"),
        author: String::from("dieter flachmann"),
        website: String::from("dieter flachmann"),
    };

    Engine::new(package_info)?.run()?;

    log::drop();

    Ok(())
}

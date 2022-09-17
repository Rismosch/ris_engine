use ris_core::engine::Engine;
use ris_data::info::app_info::AppInfo;
use ris_data::info::{package_info::PackageInfo, app_info::app_info};
use ris_data::package_info;
use ris_log::{
    appenders::{console_appender::ConsoleAppender, file_appender::FileAppender},
    log::{self, Appenders, LogGuard},
    log_level::LogLevel,
};

#[cfg(debug_assertions)]
fn init_log(app_info: &AppInfo) -> LogGuard {
    let appenders: Appenders = vec![ConsoleAppender::new(), FileAppender::new(app_info)];
    log::init(LogLevel::Trace, appenders)
}

#[cfg(not(debug_assertions))]
fn init_log(app_info: &AppInfo) -> LogGuard {
    let appenders: Appenders = vec![FileAppender::new(app_info)];
    log::init(LogLevel::Info, appenders)
}

fn main() -> Result<(), String> {
    let app_info = app_info(package_info!());
    let log_guard = init_log(&app_info);

    Ok(())

    // let result = Engine::new(app_info)?.run();

    // match result {
    //     Ok(exit_code) => ris_log::info!("exit code {}", exit_code),
    //     Err(ref error) => ris_log::fatal!("exit error \"{}\"", error),
    // }

    // drop(log_guard);

    // let exit_code = result?;
    // std::process::exit(exit_code);
}

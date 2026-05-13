use std::path::PathBuf;

use ris_data::info::app_info::AppInfo;
use ris_data::info::args_info::ArgsInfo;
use ris_data::info::build_info::BuildInfo;
use ris_data::info::cpu_info::CpuInfo;
use ris_data::info::file_info::FileInfo;
use ris_data::info::package_info::PackageInfo;
use ris_data::info::sdl_info::SdlInfo;
use ris_error::prelude::*;
use ris_log::log;
use ris_log::log::IAppender;
use ris_log::log_level::LogLevel;
use ris_log::log_message::LogMessage;

use crate::god_job;
use crate::god_object::GodObject;
use crate::log_appenders::console_appender::ConsoleAppender;
use crate::log_appenders::file_appender::FileAppender;
use crate::log_appenders::ui_helper_appender::UiHelperAppender;

pub const CLI: &str = "cli";
const LOG_LEVEL: LogLevel = LogLevel::Trace;

#[allow(clippy::large_enum_variant)]
// justification: this enum isn't hot by any means. it's created once, copied once? initializing
// the entire vulkan backend is more expensive
#[derive(Debug, Clone)]
enum EntryPoint {
    #[cfg(feature = "cli_enabled")]
    Cli(Vec<String>),
    Engine(AppInfo),
}

pub fn run(package_info: PackageInfo) -> RisResult<()> {
    let entry_point = get_entry_point(package_info.clone()).inspect_err(|e| {
        display_error(e, true);
    })?;

    let result = match entry_point.clone() {
        #[cfg(feature = "cli_enabled")]
        EntryPoint::Cli(args) => crate::cli::run(args),
        EntryPoint::Engine(app_info) => run_engine(app_info),
    };

    if let Err(e) = result.as_ref() {
        let show_popup = matches!(entry_point, EntryPoint::Engine(_));
        display_error(e, show_popup);
    }

    result
}

fn get_entry_point(package_info: PackageInfo) -> RisResult<EntryPoint> {
    let args = std::env::args().collect::<Vec<_>>();

    let is_cli_command = matches!(args.get(1).map(|x| x.as_str()), Some(CLI),);
    if is_cli_command {
        #[cfg(feature = "cli_enabled")]
        return Ok(EntryPoint::Cli(args));
        #[cfg(not(feature = "cli_enabled"))]
        return ris_error::new_result!("cli is not available");
    }

    let args_info = ArgsInfo::parse(args)?;
    let build_info = BuildInfo::new();
    let cpu_info = CpuInfo::new()?;
    let file_info = FileInfo::new(&package_info)?;
    let sdl_info = SdlInfo::new();
    let app_info = AppInfo::new(
        args_info,
        build_info,
        cpu_info,
        file_info,
        package_info,
        sdl_info,
    );

    Ok(EntryPoint::Engine(app_info))
}

fn run_engine(app_info: AppInfo) -> RisResult<()> {
    loop {
        let wants_to = {
            // setup logging
            let mut logs_dir = PathBuf::new();
            logs_dir.push(&app_info.file.pref_path);
            logs_dir.push("logs");

            let console_appender = Box::new(ConsoleAppender);
            let file_appender = Box::new(FileAppender::new(&logs_dir)?);
            let ui_helper_appender = Box::new(UiHelperAppender::new()?);
            let appenders: Vec<Box<dyn IAppender + Send>> =
                vec![console_appender, file_appender, ui_helper_appender];

            let _log_guard = log::init(LOG_LEVEL, appenders);

            ris_log::log::forward_to_appenders(LogMessage::Plain(app_info.to_string()));

            // initialize engine
            let script_registry = crate::scripts::registry()?;

            let god_object = match GodObject::new(app_info.clone(), script_registry) {
                Ok(god_object) => god_object,
                Err(e) => {
                    ris_log::fatal!("failed to create god object: {:?}", e,);
                    return Err(e);
                }
            };

            crate::scripts::setup_flycam(&god_object)?;

            // run engine
            match god_job::run(god_object) {
                Ok(result) => result,
                Err(e) => {
                    ris_log::fatal!("error during god job: {:?}", e,);
                    return Err(e);
                }
            }
        };

        // restart?
        match wants_to {
            god_job::WantsTo::Quit => return Ok(()),
            god_job::WantsTo::Restart => {
                eprintln!();
                eprintln!();
                eprintln!("restarting...");
                eprintln!();
                eprintln!();
            }
        }
    }
}

fn display_error(e: &RisError, show_popup: bool) {
    let mut message = e.to_string();
    if show_popup {
        let show_message_result = sdl2::messagebox::show_simple_message_box(
            sdl2::messagebox::MessageBoxFlag::ERROR,
            "Fatal Error",
            &message,
            None,
        );

        if let Err(e) = show_message_result {
            message.push_str(&format!("\n\nfailed to show popup: {}", e,))
        }
    }
}

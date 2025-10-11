#![cfg_attr(feature = "ris_windows_subsystem", windows_subsystem = "windows")]

#[cfg(feature = "ris_cli_enabled")]
pub mod cli;
pub mod scripts;

use std::path::PathBuf;

use ris_core::god_job;
use ris_core::god_object::GodObject;
use ris_core::log_appenders::console_appender::ConsoleAppender;
use ris_core::log_appenders::file_appender::FileAppender;
use ris_core::log_appenders::ui_helper_appender::UiHelperAppender;
use ris_data::info::app_info::AppInfo;
use ris_data::info::args_info::ArgsInfo;
use ris_data::info::build_info::BuildInfo;
use ris_data::info::cpu_info::CpuInfo;
use ris_data::info::file_info::FileInfo;
use ris_data::info::sdl_info::SdlInfo;
use ris_data::package_info;
use ris_error::prelude::*;
use ris_log::log;
use ris_log::log::IAppender;
use ris_log::log_level::LogLevel;
use ris_log::log_message::LogMessage;

const CLI: &str = "cli";
const LOG_LEVEL: LogLevel = LogLevel::Trace;
const RESTART_CODE: i32 = 42;

#[derive(Debug, Clone)]
enum EntryPoint {
    #[cfg(feature = "ris_cli_enabled")]
    Cli(Vec<String>),
    Engine(AppInfo),
    WrapProcess(AppInfo),
}

fn main() -> RisResult<()> {
    panic!("delete frame in flight file");
    panic!("parallelize renderer command buffer recording");
    panic!("free safety comments should notice that the struct shouldn't be used after free");

    let entry_point = get_entry_point().inspect_err(|e| {
        display_error(e, true);
    })?;

    let result = match entry_point.clone() {
        #[cfg(feature = "ris_cli_enabled")]
        EntryPoint::Cli(args) => cli::run(args),
        EntryPoint::Engine(app_info) => run_engine(app_info),
        EntryPoint::WrapProcess(app_info) => wrap_process(app_info),
    };

    if let Err(e) = result.as_ref() {
        let show_popup = matches!(entry_point, EntryPoint::Engine(_));
        display_error(e, show_popup);
    }

    result
}

fn get_entry_point() -> RisResult<EntryPoint> {
    let args = std::env::args().collect::<Vec<_>>();

    let is_cli_command = matches!(args.get(1).map(|x| x.as_str()), Some(CLI),);
    if is_cli_command {
        #[cfg(feature = "ris_cli_enabled")]
        return Ok(EntryPoint::Cli(args));
        #[cfg(not(feature = "ris_cli_enabled"))]
        return ris_error::new_result!("cli is not available");
    }

    let args_info = ArgsInfo::parse(args)?;
    let build_info = BuildInfo::new();
    let cpu_info = CpuInfo::new()?;
    let package_info = package_info!();
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

    let entry_point = if app_info.args.no_restart {
        EntryPoint::Engine(app_info)
    } else {
        EntryPoint::WrapProcess(app_info)
    };

    Ok(entry_point)
}

fn run_engine(app_info: AppInfo) -> RisResult<()> {
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
    let script_registry = scripts::registry()?;

    let god_object = match GodObject::new(app_info, script_registry) {
        Ok(god_object) => god_object,
        Err(e) => {
            ris_log::fatal!("failed to create god object: {:?}", e,);
            return Err(e);
        }
    };

    scripts::setup_flycam(&god_object)?;

    // run engine
    let result = match god_job::run(god_object) {
        Ok(result) => result,
        Err(e) => {
            ris_log::fatal!("error during god job: {:?}", e,);
            return Err(e);
        }
    };

    // prepare shutdown
    match result {
        god_job::WantsTo::Quit => Ok(()),
        god_job::WantsTo::Restart => std::process::exit(RESTART_CODE),
    }
}

fn wrap_process(mut app_info: AppInfo) -> RisResult<()> {
    app_info.args.no_restart = true;

    let executable_path = &app_info.args.executable_path;
    let raw_args = app_info.args.generate_raw_args();

    loop {
        let mut command = std::process::Command::new(executable_path);

        for arg in raw_args.iter().skip(1) {
            command.arg(arg);
        }

        let child = command.spawn()?;
        let output = child.wait_with_output()?;

        let exit_code = if let Some(code) = output.status.code() {
            eprintln!("process finished with code {}", code);

            if code == RESTART_CODE {
                eprintln!("restarting...\n");
                continue;
            } else {
                Some(code)
            }
        } else {
            eprintln!("process finished with no code");
            None
        };

        if output.status.success() {
            return Ok(());
        } else {
            let output_bytes = output.stderr;
            let output_string = String::from_utf8(output_bytes);

            match output_string {
                Ok(to_print) => eprintln!("{}", to_print),
                Err(error) => {
                    return ris_error::new_result!(
                        "error while formatting output.stderr: {}",
                        error
                    )
                }
            }

            match exit_code {
                Some(code) => std::process::exit(code),
                None => return Ok(()),
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

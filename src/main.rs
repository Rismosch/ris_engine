use std::path::PathBuf;

use ris_core::god_job;
use ris_core::god_object::GodObject;
use ris_data::info::app_info::AppInfo;
use ris_data::info::args_info::ArgsInfo;
use ris_data::info::build_info::BuildInfo;
use ris_data::info::cpu_info::CpuInfo;
use ris_data::info::file_info::FileInfo;
use ris_data::info::package_info::PackageInfo;
use ris_data::info::sdl_info::SdlInfo;
use ris_data::package_info;
use ris_error::RisResult;
use ris_log::appenders::console_appender::ConsoleAppender;
use ris_log::appenders::file_appender::FileAppender;
use ris_log::log;
use ris_log::log::Appenders;
use ris_log::log::LogGuard;
use ris_log::log_level::LogLevel;
use ris_log::log_message::LogMessage;

pub const RESTART_CODE: i32 = 42;

fn main() -> Result<(), String> {
    let result = match get_app_info() {
        Ok(app_info) => {
            if app_info.args.no_restart {
                run_engine(app_info)
            } else {
                wrap_process(app_info)
            }
        }
        Err(error) => Err(error),
    };

    let remapped_result = result.map_err(|e| e.to_string());

    if let Err(message) = &remapped_result {
        let _ = sdl2::messagebox::show_simple_message_box(
            sdl2::messagebox::MessageBoxFlag::ERROR,
            "Fatal Error",
            message,
            None,
        );
    }

    remapped_result
}

fn get_app_info() -> RisResult<AppInfo> {
    let args_info = ArgsInfo::new()?;
    let build_info = BuildInfo::new();
    let cpu_info = CpuInfo::new()?;
    let package_info = package_info!();
    let file_info = FileInfo::new(&package_info)?;
    let sdl_info = SdlInfo::new();

    Ok(AppInfo::new(
        args_info,
        build_info,
        cpu_info,
        file_info,
        package_info,
        sdl_info,
    ))
}

fn setup_logging(app_info: &AppInfo) -> RisResult<LogGuard> {
    let log_level = LogLevel::Debug;

    let mut logs_dir = PathBuf::new();
    logs_dir.push(&app_info.file.pref_path);
    logs_dir.push("logs");

    let console_appender = Some(ConsoleAppender);
    let file_appender = Some(FileAppender::new(&logs_dir)?);
    let appenders = Appenders {
        console_appender,
        file_appender,
    };

    let log_guard = unsafe { log::init(log_level, appenders) };

    Ok(log_guard)
}

fn run_engine(app_info: AppInfo) -> RisResult<()> {
    let _log_guard = setup_logging(&app_info)?;
    ris_log::log::forward_to_appenders(LogMessage::Plain(app_info.to_string()));

    let god_object = match GodObject::new(app_info) {
        Ok(god_object) => god_object,
        Err(e) => {
            ris_log::fatal!("failed to create god object: {:?}", e,);
            return Err(e);
        }
    };

    let result = match god_job::run(god_object) {
        Ok(result) => result,
        Err(e) => {
            ris_log::fatal!("failed to run god job: {:?}", e,);
            return Err(e);
        }
    };

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

        let child = ris_error::unroll!(command.spawn(), "child could not be spawned")?;
        let output = ris_error::unroll!(child.wait_with_output(), "child could not be awaited")?;

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
                    return ris_error::new_result!("error while formatting output.stderr: {}", error)
                }
            }

            match exit_code {
                Some(code) => std::process::exit(code),
                None => return Ok(()),
            }
        }
    }
}

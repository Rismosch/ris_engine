use ris_core::god_job;
use ris_core::god_object::GodObject;
use ris_data::gameloop::gameloop_state::GameloopState;
use ris_data::info::app_info::AppInfo;
use ris_data::info::args_info::ArgsInfo;
use ris_data::info::build_info::BuildInfo;
use ris_data::info::cpu_info::CpuInfo;
use ris_data::info::file_info::FileInfo;
use ris_data::info::package_info::PackageInfo;
use ris_data::info::sdl_info::SdlInfo;
use ris_data::package_info;
use ris_util::ris_error::RisResult;

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
    let cpu_info = CpuInfo::new();

    let args_info = ArgsInfo::new(&cpu_info)?;

    let package_info = package_info!();
    let build_info = BuildInfo::new();
    let file_info = FileInfo::new(&package_info)?;
    let sdl_info = SdlInfo::new();

    Ok(AppInfo::new(
        args_info,
        package_info,
        build_info,
        file_info,
        sdl_info,
        cpu_info,
    ))
}

fn run_engine(app_info: AppInfo) -> RisResult<()> {
    let god_object = GodObject::new(app_info)?;
    let result = god_job::run(god_object);

    match result {
        GameloopState::Error(error) => Err(error),
        GameloopState::WantsToRestart => {
            std::process::exit(RESTART_CODE);
        }
        GameloopState::WantsToQuit => Ok(()),
        GameloopState::WantsToContinue => ris_util::result_err!(
            "god job returned but wants to continue? i don't know how this is supposed to happen",
        ),
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

        let child = ris_util::unroll!(command.spawn(), "child could not be spawned")?;
        let output = ris_util::unroll!(child.wait_with_output(), "child could not be awaited")?;

        let exit_code = if let Some(code) = output.status.code() {
            println!("process finished with code {}", code);

            if code == RESTART_CODE {
                println!("restarting...\n");
                continue;
            } else {
                Some(code)
            }
        } else {
            println!("process finished with no code");
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
                    return ris_util::result_err!("error while formatting output.stderr: {}", error)
                }
            }

            match exit_code {
                Some(code) => std::process::exit(code),
                None => return ris_util::result_err!("no code to exit from"),
            }
        }
    }
}

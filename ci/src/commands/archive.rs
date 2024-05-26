use std::io::Write;
use std::path::PathBuf;

use crate::CiResult;
use crate::ICommand;

const CLEAN: &str = "clean";
const CLEAN_EVERYTHING: &str = "clean-everything";
const VENDOR: &str = "vendor";
const COMPRESS: &str = "compress";
const FORCE: &str = "-f";

const CARGO_DIR_NAME: &str = ".cargo";
const CONFIG_TOML_NAME: &str = "config.toml";

#[derive(PartialEq, Eq)]
enum Clean {
    Everything,
    ExceptVendor,
    Nothing,
}

pub struct Archive;

impl ICommand for Archive {
    fn args() -> String {
        format!(
            "[{}/{}] [{}] [{}] {}",
            CLEAN,
            CLEAN_EVERYTHING,
            VENDOR,
            COMPRESS,
            FORCE,
        )
    }

    fn explanation() -> String {
        format!("Cleans, vendors and compresses the entire workspace.")
    }

    fn run(args: Vec<String>, target_dir: PathBuf) -> CiResult<()> {

        if args.len() <= 2 {
            eprintln!("no args provided");
            crate::util::print_help_for_command(
                "archive",
                Self::args(),
                Self::explanation(),
            );
            return Ok(());
        }

        eprintln!("parsing args...");

        let mut clean = Clean::Nothing;
        let mut vendor = false;
        let mut compress = false;
        let mut force = false;

        for raw_arg in &args[2..] {
            match raw_arg.trim().to_lowercase().as_str() {
                CLEAN => clean = Clean::ExceptVendor,
                CLEAN_EVERYTHING => clean = Clean::Everything,
                VENDOR => vendor = true,
                COMPRESS => compress = true,
                FORCE => force = true,
                arg => return crate::new_error_result!("unkown arg: {}", arg)
            }
        }

        if !force {
            return crate::new_error_result!("this command deletes and changes files in the workspace. this command cannot be undone. pass `{}` to proceed anyway", FORCE);
        }

        let root_dir = crate::util::get_root_dir()?;

        match clean {
            Clean::Everything => (),
            Clean::ExceptVendor => (),
            Clean::Nothing => (),
        }

        if vendor {
            let vendor_dir = root_dir.join(VENDOR);
            if vendor_dir.exists() {
                crate::util::clean_or_create_dir(&vendor_dir)?;
            }

            let cargo_vendor = "cargo vendor";
            let mut vendor_output = String::new();
            let exit_status = crate::cmd::run(cargo_vendor, Some(&mut vendor_output))?;
            if !crate::cmd::has_exit_code(&exit_status, 0) {
                return crate::new_error_result!("failed to run `{}`", cargo_vendor);
            }

            let cargo_dir = crate::util::get_root_dir()?.join(CARGO_DIR_NAME);
            crate::util::clean_or_create_dir(&cargo_dir)?;
            let config_toml_path = cargo_dir.join(CONFIG_TOML_NAME);

            eprintln!("writing {:?}...", config_toml_path);
            let bytes = vendor_output.as_bytes();
            let mut file = std::fs::File::create(config_toml_path)?;
            file.write(bytes)?;
        }

        if !compress {
            eprintln!("done!");
        } else {

        }

        crate::new_error_result!("end")
    }
}

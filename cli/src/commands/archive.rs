use std::path::PathBuf;

use ris_error::Extensions;
use ris_error::RisResult;

use crate::ExplanationLevel;
use crate::ICommand;

const RIS_ENGINE: &str = "ris_engine";

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
            CLEAN, CLEAN_EVERYTHING, VENDOR, COMPRESS, FORCE,
        )
    }

    fn explanation(level: ExplanationLevel) -> String {
        match level {
            ExplanationLevel::Short => {
                String::from("Cleans, vendors and compresses the entire workspace.")
            }
            ExplanationLevel::Detailed => {
                let mut explanation = String::new();
                explanation.push_str("Cleans, vendors and compresses the entire workspace. This command modifies the workspace, which cannot be undone. Pass -f to proceed anyway.\n");
                explanation.push('\n');
                explanation.push_str("args:\n");
                explanation.push('\n');
                explanation.push_str(&format!("{}\n", CLEAN));
                explanation.push_str("Cleans the workspace by running a combination of git commands. Ignores vendored crates.\n");
                explanation.push_str("WARNING: Uncommitted changes will be lost!\n");
                explanation.push('\n');
                explanation.push_str(&format!("{}\n", CLEAN_EVERYTHING));
                explanation.push_str("Cleans the workspace by running a combination of git commands. Also cleans vendored crates.\n");
                explanation.push_str("WARNING: Uncommitted changes will be lost!\n");
                explanation.push('\n');
                explanation.push_str(&format!("{}\n", VENDOR));
                explanation.push_str("Vendors crates. I.e. downloads dependencies and stores them in this repo for offline use.\n");
                explanation.push('\n');
                explanation.push_str(&format!("{}\n", COMPRESS));
                explanation.push_str("Compresses the entire workspace using `7z` and `tar`.\n");
                explanation.push('\n');
                explanation.push_str(&format!("{}\n", FORCE));
                explanation.push_str("This command (cli archive) modifies the workspace, which cannot be undone. Pass -f to proceed anyway.\n");
                explanation
            }
        }
    }

    fn run(args: Vec<String>, target_dir: PathBuf) -> RisResult<()> {
        if args.len() <= 2 {
            return crate::util::command_error(
                "no args provided",
                "archive",
                Self::args(),
                Self::explanation(ExplanationLevel::Detailed),
            );
        }

        let mut clean = Clean::Nothing;
        let mut vendor = false;
        let mut compress = false;
        let mut force = false;

        for arg in &args[2..] {
            match arg.trim().to_lowercase().as_str() {
                CLEAN => clean = Clean::ExceptVendor,
                CLEAN_EVERYTHING => clean = Clean::Everything,
                VENDOR => vendor = true,
                COMPRESS => compress = true,
                FORCE => force = true,
                _ => {
                    return crate::util::command_error(
                        &format!("unkown arg: {}", arg),
                        "archive",
                        Self::args(),
                        Self::explanation(ExplanationLevel::Detailed),
                    );
                }
            }
        }

        if !force {
            return ris_error::new_result!("this command deletes and changes files in the workspace, which cannot be undone. pass `{}` to proceed anyway", FORCE);
        }

        let root_dir = crate::util::get_root_dir()?;

        if clean != Clean::Nothing {
            eprintln!("cleaning workspace...");
            crate::cmd::run("git reset .", None)?;
            crate::cmd::run("git restore .", None)?;

            match clean {
                Clean::Everything => {
                    crate::cmd::run("git clean -dxf -e target", None)?;
                }
                Clean::ExceptVendor => {
                    crate::cmd::run("git clean -dxf -e target -e vendor -e .cargo", None)?;
                }
                Clean::Nothing => (),
            }
        }

        if vendor {
            let vendor_dir = root_dir.join(VENDOR);
            if vendor_dir.exists() {
                ris_io::util::clean_or_create_dir(&vendor_dir)?;
            }

            let cargo_vendor = "cargo vendor";
            let mut vendor_output = String::new();
            let exit_status = crate::cmd::run(cargo_vendor, Some(&mut vendor_output))?;
            if !crate::cmd::has_exit_code(&exit_status, 0) {
                return ris_error::new_result!("failed to run `{}`", cargo_vendor);
            }

            let cargo_dir = crate::util::get_root_dir()?.join(CARGO_DIR_NAME);
            ris_io::util::clean_or_create_dir(&cargo_dir)?;
            let config_toml_path = cargo_dir.join(CONFIG_TOML_NAME);

            eprintln!("writing {:?}...", config_toml_path);
            let bytes = vendor_output.as_bytes();
            let mut file = std::fs::File::create(config_toml_path)?;
            ris_io::write(&mut file, bytes)?;
        }

        if !compress {
            eprintln!("done!");
            Ok(())
        } else {
            let archive_date = chrono::Local::now().format("%Y_%m_%d").to_string();

            let src_dir = root_dir.to_str().unroll()?.replace('\\', "/");
            let dst_file_path = target_dir.join(format!("{}_{}", RIS_ENGINE, archive_date));
            let dst_file_path = dst_file_path.to_str().unroll()?.replace('\\', "/");

            ris_io::util::clean_or_create_dir(&target_dir)?;

            eprintln!("compressing...");
            crate::cmd::run(&format!("7z a {}.7z {}/* -t7z -m0=lzma -mx=9 -mfb=64 -md=32m -ms=on -xr!*.git -xr!target -xr!cli_out", dst_file_path, src_dir), None)?;
            crate::cmd::run(
                &format!(
                    "7z a {}.zip {} -tzip -mx9 -mfb=258 -mpass=15 -xr!*.git -xr!target -xr!cli_out",
                    dst_file_path, src_dir
                ),
                None,
            )?;

            crate::cmd::run(
                &format!(
                    "tar --exclude=.git --exclude=target --exclude=cli_out -czf {}.tgz -C {} .",
                    dst_file_path, src_dir
                ),
                None,
            )?;

            eprintln!("done! archive can be found in {:?}", target_dir);
            Ok(())
        }
    }
}

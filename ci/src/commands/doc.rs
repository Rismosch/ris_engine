use std::path::PathBuf;

use crate::CiResult;
use crate::CiResultExtensions;
use crate::ICommand;

pub struct Doc;

impl ICommand for Doc {
    fn usage() -> String {
        format!("doc [clean]    Generates docs and moves them to another folder. Pass `clean` to run `cargo clean`.")
    }

    fn run(args: Vec<String>, target_dir: PathBuf) -> CiResult<()> {
        let mut clean = false;

        for arg in &args[2..] {
            match arg.trim().to_lowercase().as_str() {
                "clean" => clean = true,
                _ => return crate::new_error_result!("unkown arg `{}`", arg),
            }
        }

        if clean {
            crate::cmd::run("cargo clean")?;
        }

        let cargo_doc = "cargo doc";
        let exit_status = crate::cmd::run(cargo_doc)?;

        if !crate::cmd::has_exit_code(&exit_status, 0) {
            return crate::new_error_result!("`{}` failed", cargo_doc);
        }

        let doc_dir = PathBuf::from(&args[0])
            .parent()
            .to_ci_result()?
            .to_path_buf()
            .join("..")
            .join("doc");

        crate::util::clean_or_create_dir(&target_dir)?;
        eprintln!("copying files...");
        crate::util::copy_dir_all(doc_dir, &target_dir)?;

        eprintln!("done! docs can be found in {:?}", target_dir);
        let index_file = target_dir.join("ris_engine").join("index.html");
        eprintln!("you will find the index in {:?}", index_file);

        Ok(())
    }
}

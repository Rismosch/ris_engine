use std::path::Path;
use std::path::PathBuf;

use ris_error::Extensions;
use ris_error::RisResult;

use super::cmd;
use super::ExplanationLevel;
use super::ICommand;

pub struct Doc;

impl ICommand for Doc {
    fn name(&self) -> String {
        "doc".to_string()
    }

    fn args(&self) -> String {
        String::new()
    }

    fn explanation(&self, level: ExplanationLevel) -> String {
        match level {
            ExplanationLevel::Short => String::from("Generates docs and moves them to another folder."),
            ExplanationLevel::Detailed => String::from("Generates docs and moves them to another folder. This is useful, because `cargo clean` deletes the `target` dir, which includes the output of `cargo doc`. Having docs available if the workspace does not compile is invaluable."),
        }
    }

    fn run(&self, args: Vec<String>, target_dir: &Path) -> RisResult<()> {
        let cargo_doc = "cargo doc";
        let exit_status = cmd::run(cargo_doc)?;

        if !cmd::has_exit_code(&exit_status, 0) {
            return ris_error::new_result!("`{}` failed", cargo_doc);
        }

        let doc_dir = PathBuf::from(&args[0])
            .parent()
            .into_ris_error()?
            .to_path_buf()
            .join("..")
            .join("doc");

        ris_io::util::clean_or_create_dir(&target_dir)?;
        eprintln!("copying files...");
        ris_io::util::copy_dir_all(doc_dir, &target_dir)?;

        eprintln!("done! docs can be found in \"{}\"", target_dir.display(),);
        let index_file = target_dir.join("ris_engine").join("index.html");
        eprintln!("you will find the index in \"{}\"", index_file.display(),);

        Ok(())
    }
}

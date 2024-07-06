use std::path::PathBuf;

use ris_error::Extensions;
use ris_error::RisResult;

use crate::ExplanationLevel;
use crate::ICommand;

pub struct Doc;

impl ICommand for Doc {
    fn args() -> String {
        String::new()
    }

    fn explanation(level: ExplanationLevel) -> String {
        match level {
            ExplanationLevel::Short => String::from("Generates docs and moves them to another folder."),
            ExplanationLevel::Detailed => String::from("Generates docs and moves them to another folder. This is useful, because `cargo clean` deletes the `target` dir, which includes the output of `cargo doc`. Having docs available if the workspace does not compile is invaluable."),
        }
    }

    fn run(args: Vec<String>, target_dir: PathBuf) -> RisResult<()> {
        let cargo_doc = "cargo doc";
        let exit_status = crate::cmd::run(cargo_doc, None)?;

        if !crate::cmd::has_exit_code(&exit_status, 0) {
            return ris_error::new_result!("`{}` failed", cargo_doc);
        }

        let doc_dir = PathBuf::from(&args[0])
            .parent()
            .unroll()?
            .to_path_buf()
            .join("..")
            .join("doc");

        ris_file::util::clean_or_create_dir(&target_dir)?;
        eprintln!("copying files...");
        ris_file::util::copy_dir_all(doc_dir, &target_dir)?;

        eprintln!("done! docs can be found in {:?}", target_dir);
        let index_file = target_dir.join("ris_engine").join("index.html");
        eprintln!("you will find the index in {:?}", index_file);

        Ok(())
    }
}

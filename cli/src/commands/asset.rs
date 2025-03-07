use std::path::PathBuf;

use ris_asset::asset_compiler;
use ris_asset::asset_compiler::CompileOptions;
use ris_asset::asset_importer;
use ris_core::log_appenders::console_appender::ConsoleAppender;
use ris_error::Extensions;
use ris_error::RisResult;
use ris_log::log::IAppender;
use ris_log::log_level::LogLevel;

use crate::ExplanationLevel;
use crate::ICommand;

const LOG_LEVEL: LogLevel = LogLevel::Trace;

pub const COMPILE: &str = "compile";
pub const DECOMPILE: &str = "decompile";
pub const CLEAN: &str = "clean";
pub const IMPORT: &str = "import";
pub const REIMPORT: &str = "reimport";

pub enum AssetCommand {
    Compile,
    Decompile,
    Import,
}

pub struct Asset;

impl ICommand for Asset {
    fn name(&self) -> String {
        "asset".to_string()
    }

    fn args(&self) -> String {
        "<command>".to_string()
    }

    fn explanation(&self, level: ExplanationLevel) -> String {
        match level {
            ExplanationLevel::Short => String::from("Compile, decompile or import assets."),
            ExplanationLevel::Detailed => {
                let mut explanation = String::new();
                let short_explanation = self.explanation(ExplanationLevel::Short);
                explanation.push_str(&format!("{} If <source> and <target> are omitted, they will default depending on the command.\n", short_explanation));
                explanation.push('\n');
                explanation.push_str("commands:\n");
                explanation.push('\n');
                explanation.push_str(&format!("{}\n", COMPILE));
                explanation.push_str("Compiles the assets in use.\n");
                explanation.push('\n');
                explanation.push_str(&format!("{}\n", DECOMPILE));
                explanation.push_str("Decompiles the ris_assets file.\n");
                explanation.push('\n');
                explanation.push_str(&format!("{}\n", CLEAN));
                explanation.push_str("Cleans the imported assets.\n");
                explanation.push('\n');
                explanation.push_str(&format!("{}\n", IMPORT));
                explanation.push_str("Recursively imports ALL source files. Then, it copies imported files, which are marked by corresponding meta files, to the assets in use.\n");
                explanation.push('\n');
                explanation.push_str(&format!("{}\n", REIMPORT));
                explanation.push_str("Runs clean and then import.");
                explanation
            }
        }
    }

    fn run(&self, args: Vec<String>, _target_dir: PathBuf) -> RisResult<()> {
        let command = args.get(2).into_ris_error()?.to_lowercase();

        let console_appender = Box::new(ConsoleAppender);
        let appenders: Vec<Box<dyn IAppender + Send>> = vec![console_appender];
        let _log_guard = ris_log::log::init(LOG_LEVEL, appenders);

        match command.as_str() {
            COMPILE => {
                let compile_options = CompileOptions {
                    include_original_paths: false,
                };
                asset_compiler::compile(
                    asset_compiler::DEFAULT_ASSET_DIRECTORY,
                    asset_compiler::DEFAULT_COMPILED_FILE,
                    compile_options,
                )
            }
            DECOMPILE => asset_compiler::decompile(
                asset_compiler::DEFAULT_COMPILED_FILE,
                asset_compiler::DEFAULT_DECOMPILED_DIRECTORY,
            ),
            CLEAN => asset_importer::clean(asset_importer::DEFAULT_IMPORT_DIRECTORY),
            IMPORT => asset_importer::import_all(
                asset_importer::DEFAULT_SOURCE_DIRECTORY,
                asset_importer::DEFAULT_IMPORT_DIRECTORY,
                asset_importer::DEFAULT_IN_USE_DIRECTORY,
                None,
            ),
            REIMPORT => {
                asset_importer::clean(asset_importer::DEFAULT_IMPORT_DIRECTORY)?;
                asset_importer::import_all(
                    asset_importer::DEFAULT_SOURCE_DIRECTORY,
                    asset_importer::DEFAULT_IMPORT_DIRECTORY,
                    asset_importer::DEFAULT_IN_USE_DIRECTORY,
                    None,
                )
            }
            _ => ris_error::new_result!("unkown arg: {}", command),
        }
    }
}

use std::path::PathBuf;

use ris_asset::asset_compiler;
use ris_asset::asset_compiler::CompileOptions;
use ris_asset::asset_importer;
use ris_error::RisResult;
use ris_log::appenders::console_appender::ConsoleAppender;
use ris_log::log;
use ris_log::log::Appenders;
use ris_log::log_level::LogLevel;

use crate::ExplanationLevel;
use crate::ICommand;

const LOG_LEVEL: LogLevel = LogLevel::Trace;

const COMPILE: &str = "compile";
const DECOMPILE: &str = "decompile";
const IMPORTALL: &str = "import";

pub struct Asset;

impl ICommand for Asset {
    fn args() -> String {
        String::from("<command> [<source> <target>]")
    }

    fn explanation(level: ExplanationLevel) -> String {
        match level {
            ExplanationLevel::Short => String::from("Compile, decompile or import assets."),
            ExplanationLevel::Detailed => {
                let mut explanation = String::new();
                let short_explanation = Self::explanation(ExplanationLevel::Short);
                explanation.push_str(&format!("{} If <source> and <target> are omitted, they will default depending on the command.\n", short_explanation));
                explanation.push('\n');
                explanation.push_str("commands:\n");
                explanation.push('\n');
                explanation.push_str(&format!("{}\n", COMPILE));
                explanation
                    .push_str("Compiles the <source> directory into the asset file <target>.\n");
                explanation.push_str(&format!(
                    "default source: {}\n",
                    asset_compiler::DEFAULT_ASSET_DIRECTORY
                ));
                explanation.push_str(&format!(
                    "default target: {}\n",
                    asset_compiler::DEFAULT_COMPILED_FILE
                ));
                explanation.push('\n');
                explanation.push_str(&format!("{}\n", DECOMPILE));
                explanation
                    .push_str("Decompiles the asset file <source> into the directory <target>.\n");
                explanation.push_str(&format!(
                    "default source: {}\n",
                    asset_compiler::DEFAULT_COMPILED_FILE
                ));
                explanation.push_str(&format!(
                    "default target: {}\n",
                    asset_compiler::DEFAULT_DECOMPILED_DIRECTORY
                ));
                explanation.push('\n');
                explanation.push_str(&format!("{}\n", IMPORTALL));
                explanation.push_str("Recursively imports ALL files in directory <source> into the directory <target>.\n");
                explanation.push_str(&format!(
                    "default source: {}\n",
                    asset_importer::DEFAULT_SOURCE_DIRECTORY
                ));
                explanation.push_str(&format!(
                    "default target: {}\n",
                    asset_importer::DEFAULT_TARGET_DIRECTORY
                ));
                explanation
            }
        }
    }

    fn run(args: Vec<String>, _target_dir: PathBuf) -> RisResult<()> {
        let parse_result = match args.len() {
            0 | 1 => Err(String::from("too few args")),
            2 => Err(String::from("no args provided")),
            3 => Ok((&args[2], None)),
            4 => Err(String::from("no target provided")),
            5 => Ok((&args[2], Some((&args[3], &args[4])))),
            _ => Err(String::from("too many args")),
        };

        let (command, source_target) = match parse_result {
            Ok(ok) => ok,
            Err(e) => {
                return crate::util::command_error(
                    &e,
                    "asset",
                    Self::args(),
                    Self::explanation(ExplanationLevel::Detailed),
                )
            }
        };

        let console_appender = Some(ConsoleAppender);
        let file_appender = None;
        let appenders = Appenders {
            console_appender,
            file_appender,
        };

        let _log_guard = unsafe { log::init(LOG_LEVEL, appenders) };

        match command.as_str() {
            COMPILE => {
                let compile_options = CompileOptions {
                    include_original_paths: false,
                };

                match source_target {
                    Some((source, target)) => {
                        asset_compiler::compile(source, target, compile_options)
                    }
                    None => asset_compiler::compile(
                        asset_compiler::DEFAULT_ASSET_DIRECTORY,
                        asset_compiler::DEFAULT_COMPILED_FILE,
                        compile_options,
                    ),
                }
            }
            DECOMPILE => match source_target {
                Some((source, target)) => asset_compiler::decompile(source, target),
                None => asset_compiler::decompile(
                    asset_compiler::DEFAULT_COMPILED_FILE,
                    asset_compiler::DEFAULT_DECOMPILED_DIRECTORY,
                ),
            },
            IMPORTALL => match source_target {
                Some((source, target)) => asset_importer::import_all(source, target),
                None => asset_importer::import_all(
                    asset_importer::DEFAULT_SOURCE_DIRECTORY,
                    asset_importer::DEFAULT_TARGET_DIRECTORY,
                ),
            },
            command => crate::util::command_error(
                &format!("unkown command: {}", command),
                "asset",
                Self::args(),
                Self::explanation(ExplanationLevel::Detailed),
            ),
        }
    }
}

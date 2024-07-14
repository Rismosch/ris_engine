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

pub const COMPILE: &str = "compile";
pub const DECOMPILE: &str = "decompile";
pub const IMPORT: &str = "import";

pub enum AssetCommand {
    Compile,
    Decompile,
    Import,
}

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
                explanation.push_str(&format!("{}\n", IMPORT));
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
            5 => Ok((&args[2], Some((args[3].as_str(), args[4].as_str())))),
            _ => Err(String::from("too many args")),
        };

        match parse_result {
            Ok((command, source_target)) => {
                let asset_command = match command.to_lowercase().as_str() {
                    COMPILE => AssetCommand::Compile,
                    DECOMPILE => AssetCommand::Decompile,
                    IMPORT => AssetCommand::Import,
                    command => {
                        return crate::util::command_error(
                            &format!("unkown command: {}", command),
                            "asset",
                            Self::args(),
                            Self::explanation(ExplanationLevel::Detailed),
                        )
                    }
                };

                Self::execute_command(asset_command, source_target)
            }
            Err(e) => crate::util::command_error(
                &e,
                "asset",
                Self::args(),
                Self::explanation(ExplanationLevel::Detailed),
            ),
        }
    }
}

impl Asset {
    pub fn execute_command(
        command: AssetCommand,
        source_target: Option<(&str, &str)>,
    ) -> RisResult<()> {
        let console_appender = Some(ConsoleAppender);
        let file_appender = None;
        let appenders = Appenders {
            console_appender,
            file_appender,
        };

        let _log_guard = unsafe { log::init(LOG_LEVEL, appenders) };

        match command {
            AssetCommand::Compile => {
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
            AssetCommand::Decompile => match source_target {
                Some((source, target)) => asset_compiler::decompile(source, target),
                None => asset_compiler::decompile(
                    asset_compiler::DEFAULT_COMPILED_FILE,
                    asset_compiler::DEFAULT_DECOMPILED_DIRECTORY,
                ),
            },
            AssetCommand::Import => match source_target {
                Some((source, target)) => asset_importer::import_all(source, target, None),
                None => asset_importer::import_all(
                    asset_importer::DEFAULT_SOURCE_DIRECTORY,
                    asset_importer::DEFAULT_TARGET_DIRECTORY,
                    //Some("temp"),
                    None,
                ),
            },
        }
    }
}

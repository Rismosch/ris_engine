use std::env;
use std::path::PathBuf;

use ris_asset::asset_compiler;
use ris_asset::asset_importer;
use ris_log::appenders::console_appender::ConsoleAppender;
use ris_log::log;
use ris_log::log::Appenders;
use ris_log::log_level::LogLevel;

fn main() {
    let log_level = LogLevel::Trace;

    let console_appender = Some(ConsoleAppender);
    let file_appender = None;
    let appenders = Appenders {
        console_appender,
        file_appender,
    };

    let log_guard = unsafe { log::init(log_level, appenders) };

    let raw_args: Vec<String> = env::args().collect();
    if raw_args.len() != 4 && raw_args.len() != 2 {
        println!("incorrect number of argument");
        print_help();
        return;
    }

    let command_raw = &raw_args[1];
    let command = command_raw.to_lowercase();
    let source_target = if raw_args.len() == 4 {
        Some((&raw_args[2], &raw_args[3]))
    } else {
        None
    };

    let result = if command.eq("compile") {
        match source_target {
            Some((source, target)) => asset_compiler::compile(source, target),
            None => asset_compiler::compile(
                asset_compiler::DEFAULT_ASSET_DIRECTORY,
                asset_compiler::DEFAULT_COMPILED_FILE,
            ),
        }
    } else if command.eq("decompile") {
        match source_target {
            Some((source, target)) => asset_compiler::decompile(source, target),
            None => asset_compiler::decompile(
                asset_compiler::DEFAULT_COMPILED_FILE,
                asset_compiler::DEFAULT_DECOMPILED_DIRECTORY,
            ),
        }
    } else if command.eq("import") {
        match source_target {
            Some((source, target)) => {
                let info = asset_importer::DeduceImporterInfo {
                    source_file_path: PathBuf::from(source),
                    target_directory: PathBuf::from(target),
                };
                let importer_info = asset_importer::ImporterInfo::DeduceFromFileName(info);
                asset_importer::import(importer_info)
            }
            None => {
                ris_util::result_err!("import has no default values. did you mean `importall`?")
            }
        }
    } else if command.eq("importall") {
        match source_target {
            Some((source, target)) => asset_importer::import_all(source, target),
            None => asset_importer::import_all(
                asset_importer::DEFAULT_SOURCE_DIRECTORY,
                asset_importer::DEFAULT_TARGET_DIRECTORY,
            ),
        }
    } else {
        println!("unkown command: {}", command_raw);
        println!();
        print_help();
        return;
    };

    if let Err(error) = result {
        println!("error: {}", error);
        println!();
        print_help();
    }

    drop(log_guard);
}

fn print_help() {
    let name = env!("CARGO_PKG_NAME");

    println!();
    println!("correct usage: ");
    println!("  > {} <command> <source> <target>", name);
    println!();
    println!("available commands:");
    println!();
    println!("  compile");
    println!("      > {} compile <source dir> <target file>", name);
    println!();
    println!("    defaults to:");
    println!(
        "      > {} compile {} {}",
        name,
        asset_compiler::DEFAULT_ASSET_DIRECTORY,
        asset_compiler::DEFAULT_COMPILED_FILE,
    );
    println!();
    println!("  decompile");
    println!("      > {} decompile <source file> <target dir>", name);
    println!();
    println!("    defaults to:");
    println!(
        "      > {} decompile {} {}",
        name,
        asset_compiler::DEFAULT_COMPILED_FILE,
        asset_compiler::DEFAULT_DECOMPILED_DIRECTORY,
    );
    println!();
    println!("  import");
    println!("      > {} import <source file> <target file>", name);
    println!();
    println!("  importall");
    println!("      > {} importall <source dir> <target dir>", name);
    println!();
    println!("    defaults to:");
    println!(
        "      > {} importall {} {}",
        name,
        asset_importer::DEFAULT_SOURCE_DIRECTORY,
        asset_importer::DEFAULT_TARGET_DIRECTORY,
    );
    println!();
}

use std::env;
use std::path::PathBuf;

use ris_asset::asset_compiler;
use ris_asset::asset_importer;
use ris_log::console_appender::ConsoleAppender;

fn main() {
    let appenders: ris_log::log::Appenders = vec![ConsoleAppender::new()];
    let log_guard = ris_log::log::init(ris_log::log_level::LogLevel::Trace, appenders);

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
    println!("<source> and <target> depend on what <command> you've entered.");
    println!();
    println!("Available commands:");
    println!();
    println!("compile");
    println!("    <source> is the path to a directory. This directory");
    println!("    is then recursively searched and all found files will be compiled");
    println!("    into a single file to <target>. If <target> already exists, it will");
    println!("    be overwritten.");
    println!();
    println!("    If <source> and <target> are omitted, it takes following default values:");
    println!(
        "        <source> {}",
        asset_compiler::DEFAULT_ASSET_DIRECTORY
    );
    println!("        <target> {}", asset_compiler::DEFAULT_COMPILED_FILE);
    println!();
    println!("decompile");
    println!("    <source> is the path to a compiled ris_asset file.");
    println!("    <target> is a directory, where all decompiled files will be saved to.");
    println!("    If <target> already exists, it will be cleared.");
    println!();
    println!("    If <source> and <target> are omitted, it takes following default values:");
    println!("        <source> {}", asset_compiler::DEFAULT_COMPILED_FILE);
    println!(
        "        <target> {}",
        asset_compiler::DEFAULT_DECOMPILED_DIRECTORY
    );
    println!();
    println!("import");
    println!("    The importer is used to convert files into new formats, that the");
    println!("    engine is capable to use.");
    println!("    <source> is the path to a file to be imported. <target> is the path");
    println!("    where the imported file will be stored. If <target> exists, it will");
    println!("    be overwritten.");
    println!();
    println!("importall");
    println!("    Like import, but <source> and <target> are directories.");
    println!("    <target> will be searched recursively and every file will be");
    println!("    attempted to be imported. <target> will be cleared and overwritten.");
    println!();
    println!("    If <source> and <target> are omitted, it takes following default values:");
    println!(
        "        <source> {}",
        asset_importer::DEFAULT_SOURCE_DIRECTORY
    );
    println!(
        "        <target> {}",
        asset_importer::DEFAULT_TARGET_DIRECTORY
    );
    println!();
}

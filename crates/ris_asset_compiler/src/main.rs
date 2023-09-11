use std::env;
use std::path::PathBuf;

use ris_asset::asset_compiler;
use ris_asset::asset_importer;
use ris_log::console_appender::ConsoleAppender;

fn main() {
    let appenders: ris_log::log::Appenders = vec![ConsoleAppender::new()];
    let log_guard = ris_log::log::init(ris_log::log_level::LogLevel::Trace, appenders);

    let raw_args: Vec<String> = env::args().collect();
    if raw_args.len() != 4 {
        print_help();
        return;
    }

    let command_raw = &raw_args[1];
    let command = command_raw.to_lowercase();
    let source = &raw_args[2];
    let target = &raw_args[3];

    let result = if command.eq("compile") {
        asset_compiler::compile(source, target)
    } else if command.eq("decompile") {
        asset_compiler::decompile(source, target)
    } else if command.eq("import") {
        let deduce_importer_info = asset_importer::DeduceImporterInfo {
            source_file_path: PathBuf::from(source),
            target_directory: PathBuf::from(target),
        };
        let importer_info = asset_importer::ImporterInfo::DeduceFromFileName(deduce_importer_info);
        asset_importer::import(importer_info)
    } else if command.eq("importall") {
        println!("TODO not implemented yet");
        Ok(())
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
    println!("decompile");
    println!("    <source> is the path to a compiled ris_asset file.");
    println!("    <target> is a directory, where all decompiled files will be saved to.");
    println!("    If <target> already exists, it will be cleared.");
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
}

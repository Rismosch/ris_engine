use std::env;
use ris_asset::asset_compiler;
use ris_log::console_appender::ConsoleAppender;

fn main() {
    let appenders: ris_log::log::Appenders = vec![ConsoleAppender::new()];
    let log_guard = ris_log::log::init(
        ris_log::log_level::LogLevel::Trace,
        appenders,
    );

    let raw_args: Vec<String> = env::args().collect();
    if raw_args.len() != 4 {
        print_help();
        return;
    }

    let command_raw = &raw_args[1];
    let command = command_raw.to_lowercase();
    let source = raw_args[2].to_lowercase();
    let target = raw_args[3].to_lowercase();

    let result = if command.eq("compile") {
        asset_compiler::compile(&source, &target)
    } else if command.eq("decompile") {
        asset_compiler::decompile(&source, &target)
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
    println!("  > {} <compile/decompile> <source> <target>", name);
    println!();
    println!("<source> and <target> depend on whether you compile or decompile.");
    println!();
    println!("When compiling, <source> is the path to a directory. This directory");
    println!("is then recursively searched and all found files will be compiled");
    println!("into a single file to <target>. If <target> already exists, it will");
    println!("be overwritten.");
    println!();
    println!("When decompiling, <source> is the path to a compiled ris_asset file.");
    println!("<target> is a directory, where all decompiled files will be saved to.");
    println!("If <target> already exists, it will be cleared.");
}

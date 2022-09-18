use ris_core::restart_code::RESTART_CODE;
use ris_util::throw;

const PROGRAM: &str = "ris_engine.exe";

fn main() {
    loop {
        let mut command = std::process::Command::new(PROGRAM);

        for arg in std::env::args().into_iter().skip(1) {
            command.arg(arg);
        }

        match command.output() {
            Ok(output) => {
                let exit_code = if let Some(code) = output.status.code() {
                    println!("process finished with code {}", code);

                    if code == RESTART_CODE {
                        println!("restarting...");
                        continue;
                    } else {
                        Some(code)
                    }
                } else {
                    println!("process finished with no code");
                    None
                };

                if output.status.success() {
                    return;
                } else {
                    let output_bytes = output.stderr;
                    let output_string = String::from_utf8(output_bytes);

                    match output_string {
                        Ok(to_print) => eprintln!("{}", to_print),
                        Err(error) => throw!("error while formatting output.stderr: {}", error),
                    }

                    match exit_code {
                        Some(code) => std::process::exit(code),
                        None => return,
                    }
                }
            }
            Err(error) => throw!("error while running {}: {}", PROGRAM, error),
        }
    }
}

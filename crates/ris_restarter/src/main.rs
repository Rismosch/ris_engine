use ris_core::restart_code::RESTART_CODE;

fn main() {
    loop {
        const PROGRAM: &str = "ris_engine.exe";

        let mut process = std::process::Command::new(PROGRAM);

        match process.output() {
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
                    // .map_err(|e| format!("error while formatting output.stderr"));

                    match output_string {
                        Ok(to_print) => eprintln!("{}", to_print),
                        Err(error) => panic!("error while formatting output.stderr: {}", error),
                    }

                    match exit_code {
                        Some(code) => std::process::exit(code),
                        None => return,
                    }
                }
            }
            Err(error) => panic!("error while running {}: {}", PROGRAM, error),
        }
    }
}

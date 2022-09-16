use std::process::ExitStatus;

use ris_data::restarter;

fn main() -> Result<(), String> {
    loop {
        let result = start_and_run_engine();

        match result {
            Ok(exit_code) => {
                if let Some(code) = exit_code.code() {
                    if code == restarter::RESTART_CODE {
                        continue;
                    } else {
                        return Ok(());
                    }
                }
            },
            Err(error) => return Err(error),
        }
    }
}

fn start_and_run_engine() -> Result<ExitStatus, String> {
    let mut process = std::process::Command::new(restarter::ENGINE_NAME);

    process.arg("hoi");
    process.arg("poi");

    let output = process
        .output()
        .map_err(|e| format!("error while running {}: {}", restarter::ENGINE_NAME, e));
    
    match output {
        Ok(output) => Ok(output.status),
        Err(error) => Err(error),
    }
}
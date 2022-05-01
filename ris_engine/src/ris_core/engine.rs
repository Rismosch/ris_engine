use crate::ris_core::*;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let result = startup_and_run();

    shutdown();

    return result;
}

fn startup_and_run() -> Result<(), Box<dyn std::error::Error>> {
    frame_buffer::init(4);

    return gameloop::run();
}

fn shutdown() {}

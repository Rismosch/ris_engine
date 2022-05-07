use crate::frame_buffer;
use crate::gameloop;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let result = startup_and_run();

    shutdown();

    result
}

fn startup_and_run() -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        frame_buffer::init(4);
    }

    gameloop::run()
}

fn shutdown() {}

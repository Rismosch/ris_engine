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
        ris_rng::rng::init([0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15]);
    }

    gameloop::run()
}

fn shutdown() {}

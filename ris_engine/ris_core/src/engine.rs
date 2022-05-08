use crate::gameloop;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let result = startup_and_run();

    shutdown();

    result
}

fn startup_and_run() -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        ris_data::frame_buffer::init(4);
        ris_rng::rng::init()?;
    }

    gameloop::run()
}

fn shutdown() {}

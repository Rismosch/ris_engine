use crate::gameloop;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let result = startup_and_run();

    shutdown();

    result
}

fn startup_and_run() -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        ris_data::init();
        ris_rng::init()?;

        ris_sdl::init()?;

        ris_input::init();
    }

    gameloop::run()
}

fn shutdown() {}

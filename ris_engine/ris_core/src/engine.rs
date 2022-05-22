use crate::gameloop;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let result = startup_and_run();

    shutdown();

    result
}

fn startup_and_run() -> Result<(), Box<dyn std::error::Error>> {
    // let mut events = sdl_context.event_pump()?;

    unsafe {
        ris_data::frame_buffer::init(4);
        ris_rng::rng::init()?;

        ris_data::sdl2::init()?;
    }

    // let mut prev_keys = HashSet::new();
    
    gameloop::run()
}

fn shutdown() {}

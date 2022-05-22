use crate::gameloop;
use sdl-sys::*;

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

    // gameloop::run()

    let WIDTH = 640;
    let HEIGHT = 480;
    SDL_Window* window = NULL;
    SDL_Renderer* renderer = NULL;

    SDL_Init(SDL_INIT_VIDEO);
    window = SDL_CreateWindow("SDL2 Test", SDL_WINDOWPOS_UNDEFINED, SDL_WINDOWPOS_UNDEFINED, WIDTH, HEIGHT, SDL_WINDOW_SHOWN);
    renderer = SDL_CreateRenderer(window, -1, SDL_RENDERER_ACCELERATED | SDL_RENDERER_PRESENTVSYNC);

    SDL_DestroyRenderer(renderer);
    SDL_DestroyWindow(window);
    SDL_Quit();

    Ok(())
}

fn shutdown() {}

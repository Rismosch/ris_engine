use ris_core;

fn main() {
    if let Err(error) = ris_core::gameloop::run() {
        eprint!("FATAL ERROR: {}", error);
    };
}

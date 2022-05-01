mod ris_core;

fn main() {
    if let Err(error) = ris_core::gameloop::run(4) {
        eprint!("FATAL ERROR: {}", error);
    };
}

mod ris_core;

fn main() {
    let result = crate::ris_core::engine::run();

    if let Err(error) = result {
        eprint!("FATAL ERROR: {}", error);
    };
}

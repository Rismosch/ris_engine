fn main() {
    let result = ris_core::engine::run();

    if let Err(error) = result {
        eprint!("FATAL ERROR: {}", error);
    };
}

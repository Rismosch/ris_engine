use ris_core::engine::Engine;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut engine = Engine::new()?;

    engine.run()?;

    Ok(())
}

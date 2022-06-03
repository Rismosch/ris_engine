use ris_core::engine::Engine;

fn main() -> Result<(), String> {
    let mut engine = Engine::new()?;

    engine.run()?;

    Ok(())
}

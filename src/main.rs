mod app_info;

use ris_core::engine::Engine;

fn main() -> Result<(), String> {
    println!("{}", app_info::app_info());

    let mut engine = Engine::new()?;

    engine.run()?;

    Ok(())
}

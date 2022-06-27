mod info;

use ris_core::engine::Engine;

use crate::info::app_info::app_info;

fn main() -> Result<(), String> {
    println!("{}", app_info());

    let mut engine = Engine::new()?;

    engine.run()?;

    Ok(())
}

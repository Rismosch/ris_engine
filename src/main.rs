mod info;

use ris_core::engine::Engine;
use ris_log::console_appender::ConsoleAppender;

use crate::info::app_info::app_info;

fn main() -> Result<(), String> {
    let app_info = app_info();
    println!("{}", app_info);

    ris_log::log::register_appender(ConsoleAppender{});
    ris_log::log!("bruh {}", 42);
    ris_log::log!("hoi {:?}", vec![1,2,3,4,5]);

    return Ok(());

    let mut engine = Engine::new()?;

    engine.run()?;

    Ok(())
}

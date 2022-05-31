// /// # Safety
// /// Should only be called by the main thread.
// /// This method modifies global static variables, and thus is inherently unsafe.
// pub unsafe fn init() -> Result<(), Box<dyn std::error::Error>> {
//     let context = ris_sdl::context::context();
//     let game_controller_subsystenm = context.game_controller()?;

//     Ok(())
// }

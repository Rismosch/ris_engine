use crate::context::context;

static mut WINDOW: Option<Box<sdl2::video::Window>> = None;

/// # Safety
/// Should only be called by the main thread.
/// This method modifies global static variables, and thus is inherently unsafe.
pub unsafe fn init() -> Result<(), Box<dyn std::error::Error>> {
    let sdl_context = context();
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("ris_engine", 640, 480)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    WINDOW = Some(Box::new(window));

    Ok(())
}

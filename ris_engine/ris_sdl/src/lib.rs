pub mod context;
pub mod event_observer;
pub mod event_pump;
pub mod video;

/// # Safety
/// Should only be called by the main thread.
/// This method modifies global static variables, and thus is inherently unsafe.
pub unsafe fn init() -> Result<(), Box<dyn std::error::Error>> {
    context::init()?;
    video::init()?;
    event_pump::init()?;

    Ok(())
}

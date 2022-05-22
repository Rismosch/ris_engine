pub mod context;
pub mod event_pump;
pub mod video;

pub unsafe fn init() -> Result<(), Box<dyn std::error::Error>> {

    context::init()?;
    video::init()?;
    event_pump::init()?;

    Ok(())
}
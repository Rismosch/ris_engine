extern crate sdl2;

static mut CONTEXT: Option<Box<sdl2::Sdl>> = None;
static mut EVENT_PUMP: Option<Box<sdl2::EventPump>> = None;
static mut WINDOW: Option<Box<sdl2::video::Window>> = None;

/// # Safety
/// Should only be called by the main thread.
/// This method modifies global static variables, and thus is inherently unsafe.
pub unsafe fn init() -> Result<(), Box<dyn std::error::Error>> {
    let sdl_context = sdl2::init()?;

    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("ris_engine", 640, 480)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let event_pump = sdl_context.event_pump()?;

    WINDOW = Some(Box::new(window));
    EVENT_PUMP = Some(Box::new(event_pump));
    CONTEXT = Some(Box::new(sdl_context));

    Ok(())
}

pub fn context() -> &'static sdl2::Sdl {
    unsafe {
        match &CONTEXT {
            Some(sdl_context) => sdl_context.as_ref(),
            None => panic!("sdl is not initialized"),
        }
    }
}

pub fn event_pump_poll_iter() -> sdl2::event::EventPollIterator<'static> {
    unsafe {
        match &mut EVENT_PUMP {
            Some(event_pump) => event_pump.as_mut().poll_iter(),
            None => panic!("sdl is not initialized"),
        }
    }
}

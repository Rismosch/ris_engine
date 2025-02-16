use sdl2_sys::SDL_Event;

/// Safety: SDL2 must be in a valid state
///
/// because the bullshit sdl2 rust binding doesn't make its ptr conversion public, and i don't want
/// to spent hours reimplementing it, we skip the "safe" polling and just raw dog the pointers.
/// fuck this, seriously
pub unsafe fn poll_sdl2_event() -> Option<SDL_Event> {
    // sdl_event
    let mut raw = std::mem::MaybeUninit::uninit();
    let has_pending = sdl2_sys::SDL_PollEvent(raw.as_mut_ptr()) == 1;
    if !has_pending {
        return None;
    }

    let event = raw.assume_init();
    Some(event)
}

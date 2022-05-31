pub mod game_controller;
pub mod gate;
pub mod keyboard;
pub mod mouse;
pub mod rebind;
pub mod util;

/// # Safety
/// Should only be called by the main thread.
/// This method modifies global static variables, and thus is inherently unsafe.
pub unsafe fn init() {
    // keyboard::init();
    // mouse::init();
    // rebind::init();
}

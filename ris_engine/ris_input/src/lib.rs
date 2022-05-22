pub mod gate;
pub mod keyboard;

pub unsafe fn init() {
    keyboard::init();
}
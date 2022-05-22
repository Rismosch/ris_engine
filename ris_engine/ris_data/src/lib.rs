pub mod frame;
pub mod frame_buffer;
pub mod sdl2;

pub unsafe fn init() {
    frame_buffer::init(4);
}
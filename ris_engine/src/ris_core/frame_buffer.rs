use crate::ris_core::*;

struct FrameBuffer {
    frames: Vec<frame::Frame>,
    current: usize,
}

static mut FRAME_BUFFER: Option<FrameBuffer> = None;

pub fn init(frame_buffer_lenght: usize)
{
    unsafe {
        let frames = Vec::with_capacity(frame_buffer_lenght);

        FRAME_BUFFER = Some(FrameBuffer{frames, current: 0});

        let number_offset = (0 - (frame_buffer_lenght as isize)) as usize;
        for i in 0..frame_buffer_lenght {
            FRAME_BUFFER.unwrap().frames.push(frame::Frame::new(frame::IDEAL_DELTA, number_offset + i));
        }
    }
}

pub fn get_previous(){

}
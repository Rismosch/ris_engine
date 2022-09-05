use std::time::Duration;

#[derive(Clone, Copy)]
pub struct FrameData {
    delta: Duration,
    number: usize,
}

impl FrameData {
    pub fn new() -> Self {
        FrameData { delta: Duration::ZERO, number: 0 }
    }

    pub fn bump(&mut self) {

    }
}
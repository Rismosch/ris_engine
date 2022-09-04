use std::time::Duration;

pub struct FrameData {
    delta: Duration,
    number: usize,
}

impl FrameData {
    pub fn new() -> Self {
        FrameData { delta: Duration::ZERO, number: 0 }
    }
}
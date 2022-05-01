use std::time::Duration;

const MAX_DELTA: Duration = Duration::from_secs(1);
pub const IDEAL_DELTA: Duration = Duration::from_millis(1000 / 60);

pub struct Frame {
    pub delta: Duration,
    pub number: usize,
}

impl Frame{
    pub fn new(delta: Duration, number: usize) -> Frame{
        if delta > MAX_DELTA {
            Frame {delta: IDEAL_DELTA, number}
        } else {
            Frame {delta, number}
        }
    }
}
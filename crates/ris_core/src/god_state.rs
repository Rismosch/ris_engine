use std::collections::VecDeque;
use std::sync::Arc;
use std::sync::Mutex;

#[derive(Default)]
pub struct InnerStateBuffer {
    pub command_queue: VecDeque<()>,
    pub event_list: Vec<()>,
    pub data: Vec<i32>,
}

pub type StateBuffer = Arc<Mutex<InnerStateBuffer>>;

#[derive(Default)]
pub struct GodState {
    double_buffer: (StateBuffer, StateBuffer),
}

impl GodState {
    pub fn swap(&mut self) {
        std::mem::swap(&mut self.double_buffer.0, &mut self.double_buffer.1);
    }

    pub fn front(&self) -> StateBuffer {
        self.double_buffer.0.clone()
    }

    pub fn back(&self) -> StateBuffer {
        self.double_buffer.1.clone()
    }
}

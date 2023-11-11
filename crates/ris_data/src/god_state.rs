use std::collections::VecDeque;
use std::sync::Arc;
use std::sync::Mutex;

#[derive(Default)]
pub struct InnerGodState {
    pub data: GodStateData,
    pub command_queue: VecDeque<GodStateCommand>,
    pub events: GodStateEvents,
}

#[derive(Default)]
pub struct GodStateData {
    pub debug: i32,
}

#[derive(Clone)]
pub enum GodStateCommand {
    IncreaseDebug,
    DecreaseDebug,
}

#[derive(Default)]
pub struct GodStateEvents {
    pub debug_increased: bool,
    pub debug_decreased: bool,
}

pub type GodState = Arc<Mutex<InnerGodState>>;

#[derive(Default)]
pub struct GodStateDoubleBuffer {
    double_buffer: (GodState, GodState),
}

impl GodStateDoubleBuffer {
    pub fn swap(&mut self) {
        std::mem::swap(&mut self.double_buffer.0, &mut self.double_buffer.1);
    }

    pub fn front(&self) -> GodState {
        self.double_buffer.0.clone()
    }

    pub fn back(&self) -> GodState {
        self.double_buffer.1.clone()
    }
}

pub fn execute_god_state_command(
    god_state: &mut InnerGodState,
    command: GodStateCommand,
    generate_events: bool) {
    match command {
        GodStateCommand::IncreaseDebug => {
            god_state.data.debug += 1;
            if generate_events {
                god_state.events.debug_increased = true;
            }
        },
        GodStateCommand::DecreaseDebug => {
            god_state.data.debug -= 1;
            if generate_events {
                god_state.events.debug_decreased = true;
            }
        },
    }
}

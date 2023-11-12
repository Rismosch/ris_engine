use std::cell::RefCell;
use std::mem::MaybeUninit;
use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

use ris_util::throw;

// if you come even close to this number, you are doing something wrong
const MAX_COMMAND_COUNT: usize = 1_000;

pub struct GodStateQueue {
    array: [RefCell<GodStateCommand>; MAX_COMMAND_COUNT],
    count: AtomicUsize,
    iter_index: AtomicUsize,
}

impl Default for GodStateQueue {
    fn default() -> Self {
        let uninitialized_array: [MaybeUninit<GodStateCommand>; MAX_COMMAND_COUNT] =
            unsafe { MaybeUninit::uninit().assume_init() };
        let array = uninitialized_array.map(|_| RefCell::new(GodStateCommand::IncreaseDebug));

        let count = AtomicUsize::new(0);
        let iter_index = AtomicUsize::new(0);

        Self {
            array,
            count,
            iter_index,
        }
    }
}

impl GodStateQueue {
    pub fn push(&self, command: GodStateCommand) {
        let index = self.count.fetch_add(1, Ordering::SeqCst);
        if index >= MAX_COMMAND_COUNT {
            throw!("{} commands exceeded", MAX_COMMAND_COUNT);
        }

        let mut element = self.array[index].borrow_mut();
        *element = command;
    }

    pub fn clear(&self) {
        self.count.store(0, Ordering::SeqCst);
    }

    pub fn start_iter(&self) {
        self.iter_index.store(0, Ordering::SeqCst);
    }

    pub fn next(&self) -> Option<GodStateCommand> {
        let index = self.iter_index.fetch_add(1, Ordering::SeqCst);
        let max_index = self.count.load(Ordering::SeqCst) as isize - 1;
        if index as isize > max_index {
            None
        } else {
            let element = self.array[index].borrow();
            let result = (*element).clone();
            Some(result)
        }
    }
}

unsafe impl Send for GodStateQueue {}
unsafe impl Sync for GodStateQueue {}

#[derive(Default)]
pub struct InnerGodState {
    pub data: GodStateData,
    pub command_queue: GodStateQueue,
    pub events: GodStateEvents,
}

pub type GodState = Arc<RefCell<InnerGodState>>;

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

#[derive(Default)]
pub struct GodStateDoubleBuffer {
    pub front: GodState,
    pub back: GodState,
    pub prev_queue: GodStateQueue,
}

impl GodStateDoubleBuffer {
    pub fn swap_and_reset(&mut self){
        let mut front = self.front.borrow_mut();
        let mut back = self.back.borrow_mut();

        std::mem::swap(&mut front.data, &mut back.data);
        std::mem::swap(&mut front.events, &mut back.events);

        std::mem::swap(&mut front.command_queue, &mut back.command_queue);
        std::mem::swap(&mut back.command_queue, &mut self.prev_queue);

        back.events = GodStateEvents::default();
        front.command_queue.clear();
    }
}

pub fn execute_god_state_command(
    state: &mut InnerGodState,
    command: GodStateCommand,
    generate_events: bool,
) {
    match command {
        GodStateCommand::IncreaseDebug => {
            state.data.debug += 1;
            if generate_events {
                state.events.debug_increased = true;
            }
        }
        GodStateCommand::DecreaseDebug => {
            state.data.debug -= 1;
            if generate_events {
                state.events.debug_decreased = true;
            }
        }
    }
}

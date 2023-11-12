use std::mem::MaybeUninit;
use std::collections::VecDeque;
use std::marker::PhantomData;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::sync::Mutex;

// arbitrary high number. i started to experience lag somewhere above 100_000 commands. try to keep
// it well below that.
const MAX_ITEM_COUNT: usize = 1_000;

type GodStateArray = [GodStateCommand; MAX_ITEM_COUNT];

pub struct GodStateQueue {
    data: *mut GodStateCommand,
    count: AtomicUsize,
    iter_index: AtomicUsize,
    _boo: PhantomData<GodStateArray>,
}

impl Default for GodStateQueue {
    fn default() -> Self {
        let uninitialized_array: [MaybeUninit<GodStateCommand>; MAX_ITEM_COUNT] = unsafe {
            MaybeUninit::uninit().assume_init()
        };
        let data_array = uninitialized_array.map(|_| GodStateCommand::IncreaseDebug);
        let data_box = Box::new(data_array);
        let data = Box::leak(data_box).as_mut_ptr();

        let count = AtomicUsize::new(0);
        let iter_index = AtomicUsize::new(0);

        Self {
            data,
            count,
            iter_index,
            _boo: PhantomData,
        }
    }
}

impl Drop for GodStateQueue {
    fn drop(&mut self) {
        let data_array = self.data as *mut [GodStateCommand; MAX_ITEM_COUNT];
        let _data_box = unsafe {Box::from_raw(data_array)};
    }
}

impl GodStateQueue {
    pub fn push(&self, command: GodStateCommand) {
        let index = self.count.fetch_add(1, Ordering::SeqCst);
        if index >= MAX_ITEM_COUNT {
            panic!("{} commands exceeded", MAX_ITEM_COUNT);
        }

        let element = self.data.wrapping_add(index);
        unsafe {*element = command;}
    }

    pub fn clear(&self) {
        self.count.store(0, Ordering::SeqCst);
    }

    pub fn start_iter(&self) {
        self.iter_index.store(0, Ordering::SeqCst);
    }

    /// # Safety
    ///
    /// Causes undefined behaviour, when other references to the queue exist, particularily if
    /// someone is still pushing to it. Only call this method, when everyone is finished pushing.
    pub unsafe fn next(&self) -> Option<&GodStateCommand> {
        let index = self.iter_index.fetch_add(1, Ordering::SeqCst);
        let max_index = usize::saturating_sub(self.count.load(Ordering::SeqCst), 1);
        if index > max_index {
            None
        } else {
            let element = self.data.wrapping_add(index);
            let result = unsafe {&*element};
            Some(result)
        }
    }
}

unsafe impl Send for GodStateQueue {}
unsafe impl Sync for GodStateQueue {}





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

pub enum GodStateCommand {
    IncreaseDebug,
    DecreaseDebug,
    NumberDebug(i32),
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
        GodStateCommand::NumberDebug(_) => (),
    }
}

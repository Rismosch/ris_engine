use std::cell::UnsafeCell;
use std::marker::PhantomData;
use std::ops::Deref;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

use ris_util::throw;

use crate::settings::Settings;

pub struct GodStateData {
    pub debug: i32,
    pub settings: Settings,
}

#[derive(Clone, PartialEq, Eq)]
pub enum GodStateCommand {
    SetJobWorkersSetting(Option<usize>),
    SaveSettings,
}

#[derive(Default)]
pub struct GodStateEvents {
    pub job_workers_settings_changed: bool,
    pub save_settings_requested: bool,
}

pub struct InnerGodState {
    pub data: GodStateData,
    pub command_queue: GodStateQueue,
    pub events: GodStateEvents,
}

impl InnerGodState {
    pub fn new(settings: Settings) -> UnsafeCell<Self> {
        let data = GodStateData {
            settings,
            debug: i32::default(),
        };
        let command_queue = GodStateQueue::default();
        let events = GodStateEvents::default();

        UnsafeCell::new(Self {
            data,
            command_queue,
            events,
        })
    }

    pub fn execute_command(&mut self, command: GodStateCommand, generate_events: bool) {
        match command {
            GodStateCommand::SetJobWorkersSetting(workers) => {
                self.data.settings.job.workers = workers;
                if generate_events {
                    self.events.job_workers_settings_changed = true;
                }
            }
            GodStateCommand::SaveSettings => {
                if generate_events {
                    self.events.save_settings_requested = true;
                }
            }
        }
    }
}

// arbitrary high number. if you are ever comming close to this number of commands, you are
// probably doing something wrong. i highly discourage you to increase this value. Somewhere
// above 100_000 i started experiencing lag.
const MAX_COMMAND_COUNT: usize = 1_000;

pub struct GodStateQueue {
    data: Vec<UnsafeCell<GodStateCommand>>,
    count: AtomicUsize,
    iter_index: AtomicUsize,
}

impl Default for GodStateQueue {
    fn default() -> Self {
        let mut data = Vec::with_capacity(MAX_COMMAND_COUNT);
        for _ in 0..MAX_COMMAND_COUNT {
            let item = UnsafeCell::new(GodStateCommand::SetJobWorkersSetting(None));
            data.push(item);
        }

        let count = AtomicUsize::new(0);
        let iter_index = AtomicUsize::new(0);

        Self {
            data,
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

        let element = self.data[index].get();
        unsafe { *element = command };
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
            let element_ptr = self.data[index].get();
            let element = unsafe { element_ptr.as_ref() }.unwrap();
            let result = element.clone();
            Some(result)
        }
    }
}

unsafe impl Send for GodStateQueue {}
unsafe impl Sync for GodStateQueue {}

#[derive(Clone)]
pub struct GodStateRef {
    ptr: *const InnerGodState,
    _boo: PhantomData<InnerGodState>,
}

impl GodStateRef {
    /// # Safety
    ///
    /// `deref` performs an `unsafe` operation. For ergonomics, the method is declared safe. To avoid UB, the construction of a `GodStateRef` is `unsafe` instead.
    ///
    /// You must make sure, as long as `GodStateRef` is alive, that:
    /// - `ptr` is live and never deallocated.
    /// - The `InnerGodState` to which `ptr` is pointing to is never modified.
    pub unsafe fn from(ptr: *const InnerGodState) -> Self {
        Self {
            ptr,
            _boo: PhantomData,
        }
    }
}

impl Deref for GodStateRef {
    type Target = InnerGodState;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ptr }
    }
}

unsafe impl Send for GodStateRef {}
unsafe impl Sync for GodStateRef {}

pub struct GodStateDoubleBuffer {
    pub front: UnsafeCell<InnerGodState>,
    pub back: UnsafeCell<InnerGodState>,
    pub prev_queue: GodStateQueue,
}

impl GodStateDoubleBuffer {
    pub fn swap_and_reset(&mut self) {
        let front = self.front.get_mut();
        let back = self.back.get_mut();

        std::mem::swap(&mut front.data, &mut back.data);
        std::mem::swap(&mut front.events, &mut back.events);

        std::mem::swap(&mut front.command_queue, &mut back.command_queue);
        std::mem::swap(&mut front.command_queue, &mut self.prev_queue);

        back.events = GodStateEvents::default();
        front.command_queue.clear();
    }
}

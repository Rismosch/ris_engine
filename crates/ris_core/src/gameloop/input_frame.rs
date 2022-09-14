use std::cell::UnsafeCell;

use ris_data::gameloop::{
    frame_data::FrameData, gameloop_state::GameloopState, input_data::InputData,
};
use ris_input::{
    keyboard_logic::update_keyboard,
    mouse_logic::{post_update_mouse, pre_update_mouse, update_mouse},
};
use ris_jobs::job_system;
use sdl2::{event::Event, EventPump};

pub struct InputFrame {
    event_pump: UnsafeCell<EventPump>,
}

impl InputFrame {
    pub fn new(event_pump: EventPump) -> Self {
        Self {
            event_pump: UnsafeCell::new(event_pump),
        }
    }

    pub fn run(
        &mut self,
        mut current: InputData,
        previous: &'static InputData,
        _frame: &FrameData,
    ) -> (InputData, GameloopState) {
        let mut current_mouse = current.take_mouse();
        let current_keyboard = current.take_keyboard();

        pre_update_mouse(&mut current_mouse);

        {
            let event_pump = unsafe { &mut *self.event_pump.get() };

            for event in event_pump.poll_iter() {
                ris_log::trace!("fps: {} event: {:?}", _frame.fps(), event);

                if let Event::Quit { .. } = event {
                    return (current, GameloopState::WantsToQuit);
                };

                update_mouse(&mut current_mouse, &event);
            }
        }

        {
            let mouse_event_pump = unsafe { &*self.event_pump.get() };
            let keyboard_event_pump = unsafe { &*self.event_pump.get() };

            let keyboard_future = job_system::submit(|| {
                let mut keyboard = current_keyboard;

                update_keyboard(
                    &mut keyboard,
                    previous.get_keyboard(),
                    keyboard_event_pump.keyboard_state(),
                );

                keyboard
            });

            post_update_mouse(
                &mut current_mouse,
                previous.get_mouse(),
                mouse_event_pump.mouse_state(),
            );

            let new_keyboard = keyboard_future.wait();

            current.set_mouse(current_mouse);
            current.set_keyboard(new_keyboard);

            (current, GameloopState::WantsToContinue)
        }
    }
}

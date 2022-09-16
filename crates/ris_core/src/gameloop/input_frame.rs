use std::cell::UnsafeCell;

use ris_data::gameloop::{
    frame_data::FrameData, gameloop_state::GameloopState, input_data::InputData,
};
use ris_input::{
    gamepad_logic::update_gamepad,
    keyboard_logic::update_keyboard,
    mouse_logic::{handle_mouse_events, post_update_mouse, reset_mouse_refs},
};
use ris_jobs::job_system;
use sdl2::{controller::GameController, event::Event, EventPump, GameControllerSubsystem};

pub struct InputFrame {
    event_pump: UnsafeCell<EventPump>,
    controller: Option<GameController>,
    controller_subsystem: UnsafeCell<GameControllerSubsystem>,
}

impl InputFrame {
    pub fn new(event_pump: EventPump, controller_subsystem: GameControllerSubsystem) -> Self {
        Self {
            event_pump: UnsafeCell::new(event_pump),
            controller: None,
            controller_subsystem: UnsafeCell::new(controller_subsystem),
        }
    }

    pub fn run(
        &mut self,
        mut current: InputData,
        previous: &'static InputData,
        _frame: &FrameData,
    ) -> (InputData, GameloopState) {
        let mut current_mouse = current.mouse;
        let current_keyboard = current.keyboard;
        let current_gamepad = current.gamepad;
        let current_controller = self.controller.take();

        reset_mouse_refs(&mut current_mouse);

        {
            let event_pump = unsafe { &mut *self.event_pump.get() };

            for event in event_pump.poll_iter() {
                ris_log::trace!("fps: {} event: {:?}", _frame.fps(), event);

                if let Event::Quit { .. } = event {
                    current.mouse = current_mouse;
                    current.keyboard = current_keyboard;
                    current.gamepad = current_gamepad;
                    self.controller = current_controller;
                    return (current, GameloopState::WantsToQuit);
                };

                handle_mouse_events(&mut current_mouse, &event);
            }
        }

        {
            let mouse_event_pump = unsafe { &*self.event_pump.get() };
            let keyboard_event_pump = unsafe { &*self.event_pump.get() };
            let controller_subsystem = unsafe { &*self.controller_subsystem.get() };

            let gamepad_future = job_system::submit(|| {
                let mut gamepad = current_gamepad;

                let new_controller = update_gamepad(
                    &mut gamepad,
                    &previous.gamepad,
                    current_controller,
                    controller_subsystem,
                );

                (gamepad, new_controller)
            });

            let keyboard_future = job_system::submit(|| {
                let mut keyboard = current_keyboard;

                let gameloop_state = update_keyboard(
                    &mut keyboard,
                    &previous.keyboard,
                    keyboard_event_pump.keyboard_state(),
                );

                (keyboard, gameloop_state)
            });

            post_update_mouse(
                &mut current_mouse,
                &previous.mouse,
                mouse_event_pump.mouse_state(),
            );

            let (new_gamepad, new_controller) = gamepad_future.wait();
            let (new_keyboard, new_gameloop_state) = keyboard_future.wait();

            current.mouse = current_mouse;
            current.keyboard = new_keyboard;
            current.gamepad = new_gamepad;

            self.controller = new_controller;

            (current, new_gameloop_state)
        }
    }
}

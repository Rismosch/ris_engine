use ris_data::{
    gameloop::{frame_data::FrameData, gameloop_state::GameloopState, input_data::InputData},
    input::rebind_matrix::RebindMatrix,
};
use ris_input::{
    gamepad_logic::update_gamepad,
    general_logic::{update_general, GeneralLogicArgs},
    keyboard_logic::update_keyboard,
    mouse_logic::{handle_mouse_events, post_update_mouse, reset_mouse_refs},
};
use ris_jobs::{
    job_cell::{JobCell, Ref},
    job_system,
};
use sdl2::{controller::GameController, event::Event, EventPump, GameControllerSubsystem};

pub struct InputFrame {
    event_pump: JobCell<EventPump>,
    controller: Option<GameController>,
    controller_subsystem: JobCell<GameControllerSubsystem>,

    rebind_matrix_mouse: RebindMatrix,
    rebind_matrix_keyboard: RebindMatrix,
    rebind_matrix_gamepad: RebindMatrix,
}

impl InputFrame {
    pub fn new(event_pump: EventPump, controller_subsystem: GameControllerSubsystem) -> Self {
        let mut rebind_matrix = [0; 32];
        for (i, row) in rebind_matrix.iter_mut().enumerate() {
            *row = 1 << i;
        }

        Self {
            event_pump: JobCell::new(event_pump),
            controller: None,
            controller_subsystem: JobCell::new(controller_subsystem),
            rebind_matrix_mouse: rebind_matrix,
            rebind_matrix_keyboard: rebind_matrix,
            rebind_matrix_gamepad: rebind_matrix,
        }
    }

    pub fn run(
        &mut self,
        mut current: InputData,
        previous: Ref<InputData>,
        _frame: Ref<FrameData>,
    ) -> (InputData, GameloopState) {
        let current_keyboard = current.keyboard;
        let current_gamepad = current.gamepad;

        let current_controller = self.controller.take();

        let previous_for_mouse = previous.clone();
        let previous_for_keyboard = previous.clone();
        let previous_for_gamepad = previous.clone();
        let previous_for_general = previous;

        reset_mouse_refs(&mut current.mouse);

        for event in self.event_pump.as_mut().poll_iter() {
            ris_log::trace!("fps: {} event: {:?}", _frame.fps(), event);

            if let Event::Quit { .. } = event {
                current.keyboard = current_keyboard;
                current.gamepad = current_gamepad;
                self.controller = current_controller;
                return (current, GameloopState::WantsToQuit);
            };

            handle_mouse_events(&mut current.mouse, &event);
        }

        let mouse_event_pump = self.event_pump.borrow();
        let keyboard_event_pump = self.event_pump.borrow();
        let controller_subsystem = self.controller_subsystem.borrow();

        let gamepad_future = job_system::submit(move || {
            let mut gamepad = current_gamepad;

            let new_controller = update_gamepad(
                &mut gamepad,
                &previous_for_gamepad.gamepad,
                current_controller,
                &*controller_subsystem,
            );

            (gamepad, new_controller)
        });

        let keyboard_future = job_system::submit(move || {
            let mut keyboard = current_keyboard;

            let gameloop_state = update_keyboard(
                &mut keyboard,
                &previous_for_keyboard.keyboard,
                keyboard_event_pump.keyboard_state(),
            );

            (keyboard, gameloop_state)
        });

        post_update_mouse(
            &mut current.mouse,
            &previous_for_mouse.mouse,
            mouse_event_pump.mouse_state(),
        );

        let (new_gamepad, new_controller) = gamepad_future.wait();
        let (new_keyboard, new_gameloop_state) = keyboard_future.wait();

        let args = GeneralLogicArgs {
            new_general_data: &mut current.general,
            old_general_data: &previous_for_general.general,
            mouse: &current.mouse.buttons,
            keyboard: &new_keyboard.buttons,
            gamepad: &new_gamepad.buttons,
            rebind_matrix_mouse: &self.rebind_matrix_mouse,
            rebind_matrix_keyboard: &self.rebind_matrix_keyboard,
            rebind_matrix_gamepad: &self.rebind_matrix_gamepad,
        };

        update_general(args);

        current.keyboard = new_keyboard;
        current.gamepad = new_gamepad;

        self.controller = new_controller;

        (current, new_gameloop_state)
    }
}

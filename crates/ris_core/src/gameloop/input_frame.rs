use ris_data::gameloop::frame_data::FrameData;
use ris_data::gameloop::gameloop_state::GameloopState;
use ris_data::gameloop::input_data::InputData;
use ris_data::input::rebind_matrix::RebindMatrix;
use ris_input::gamepad_logic::GamepadLogic;
use ris_input::general_logic::update_general;
use ris_input::general_logic::GeneralLogicArgs;
use ris_input::keyboard_logic::update_keyboard;
use ris_input::mouse_logic::handle_mouse_events;
use ris_input::mouse_logic::post_update_mouse;
use ris_input::mouse_logic::reset_mouse_refs;
use ris_jobs::job_cell::JobCell;
use ris_jobs::job_system;
use sdl2::event::Event;
use sdl2::event::WindowEvent;
use sdl2::EventPump;
use sdl2::GameControllerSubsystem;

pub struct InputFrame {
    event_pump: JobCell<EventPump>,

    gamepad_logic: Option<GamepadLogic>,

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
            event_pump: unsafe { JobCell::new(event_pump) },
            gamepad_logic: Some(GamepadLogic::new(controller_subsystem)),
            rebind_matrix_mouse: rebind_matrix,
            rebind_matrix_keyboard: rebind_matrix,
            rebind_matrix_gamepad: rebind_matrix,
        }
    }

    pub fn run(
        &mut self,
        current: &mut InputData,
        previous: &InputData,
        _frame: &FrameData,
    ) -> GameloopState {
        let current_keyboard = std::mem::take(&mut current.keyboard);
        let current_gamepad = std::mem::take(&mut current.gamepad);

        let previous_for_mouse = previous.clone();
        let previous_for_keyboard = previous.clone();
        let previous_for_gamepad = previous.clone();
        let previous_for_general = previous;

        let mut gamepad_logic = match self.gamepad_logic.take() {
            Some(gamepad_logic) => gamepad_logic,
            None => unreachable!(),
        };

        reset_mouse_refs(&mut current.mouse);

        current.window_size_changed = None;

        for event in self.event_pump.as_mut().poll_iter() {
            // ris_log::trace!("fps: {} event: {:?}", _frame.fps(), event);

            if let Event::Quit { .. } = event {
                current.keyboard = current_keyboard;
                current.gamepad = current_gamepad;
                return GameloopState::WantsToQuit;
            };

            if let Event::Window {
                win_event: WindowEvent::SizeChanged(w, h),
                ..
            } = event
            {
                current.window_size_changed = Some((w, h));
                ris_log::trace!("window changed size to {}x{}", w, h);
            }

            if handle_mouse_events(&mut current.mouse, &event) {
                continue;
            }

            if gamepad_logic.handle_events(&event) {
                continue;
            }
        }

        let mouse_event_pump = self.event_pump.borrow();
        let keyboard_event_pump = self.event_pump.borrow();

        let gamepad_future = job_system::submit(move || {
            let mut current_gamepad = current_gamepad;
            let mut gamepad_logic = gamepad_logic;

            gamepad_logic.update(&mut current_gamepad, &previous_for_gamepad.gamepad);

            (current_gamepad, gamepad_logic)
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

        let (new_gamepad, new_gamepad_logic) = gamepad_future.wait();
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
        self.gamepad_logic = Some(new_gamepad_logic);

        new_gameloop_state
    }
}

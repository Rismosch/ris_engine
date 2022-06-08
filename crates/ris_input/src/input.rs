use sdl2::{event::Event, keyboard::Scancode, EventPump, Sdl};

use crate::{
    gamepad::{Gamepad, IGamepad},
    general::{General, IGeneral},
    keyboard::{IKeyboard, Keyboard},
    mouse::{IMouse, Mouse},
};

pub struct Input {
    mouse: Mouse,
    keyboard: Keyboard,
    gamepad: Gamepad,
    general: General,
}

pub trait IInput {
    fn mouse(&self) -> &dyn IMouse;
    fn keyboard(&self) -> &dyn IKeyboard;
    fn gamepad(&self) -> &dyn IGamepad;
    fn general(&self) -> &dyn IGeneral;

    fn pre_update(&mut self);
    fn update(&mut self, event: &Event);
    fn post_update(&mut self, event_pump: &EventPump);
}

impl Input {
    pub fn new(sdl_context: &Sdl) -> Result<Input, String> {
        let mouse = Mouse::default();
        let mut keyboard = Keyboard::default();
        let gamepad = Gamepad::new(sdl_context)?;
        let general = General::default();

        let mut keymask = [Scancode::Space; 32];
        keymask[0] = Scancode::W;
        keymask[1] = Scancode::A;
        keymask[2] = Scancode::S;
        keymask[3] = Scancode::D;

        keyboard.set_keymask(keymask);

        let input = Input {
            mouse,
            keyboard,
            gamepad,
            general,
        };

        Ok(input)
    }
}

impl IInput for Input {
    fn mouse(&self) -> &'static (dyn IMouse + '_) {
        &self.mouse
    }

    fn keyboard(&self) -> &'static (dyn IKeyboard + '_) {
        &self.keyboard
    }

    fn gamepad(&self) -> &'static (dyn IGamepad + '_) {
        &self.gamepad
    }

    fn general(&self) -> &'static (dyn IGeneral + '_) {
        &self.general
    }

    fn pre_update(&mut self) {
        self.mouse.pre_update();
    }

    fn update(&mut self, event: &Event) {
        self.mouse.update(event);
    }

    fn post_update(&mut self, event_pump: &EventPump) {
        self.mouse.update_state(event_pump.mouse_state());
        self.keyboard.update_state(event_pump.keyboard_state());
        self.gamepad.update_state();

        self.general.update_state(
            self.mouse.buttons(),
            self.keyboard.buttons(),
            self.gamepad.buttons(),
        );
    }
}

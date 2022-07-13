use sdl2::{controller::GameController, GameControllerSubsystem, Sdl};

use crate::{
    buttons::{Buttons, IButtons},
    gamepad_util::{get_button_index, ALL_BUTTONS},
};

pub struct Gamepad {
    buttons: Buttons,
    axis: [i16; 6],

    subsystem: GameControllerSubsystem,
    game_controller: Option<GameController>,

    deadzone_stick: i16,
    deadzone_trigger: i16,
    axis_button_threshhold: i16,
}

pub trait IGamepad {
    fn buttons(&self) -> &Buttons;
    fn axis(&self) -> &[i16; 6];

    fn deadzone_stick(&self) -> i16;
    fn set_deadzone_stick(&mut self, value: &i16);
    fn deadzone_trigger(&self) -> i16;
    fn set_deadzone_trigger(&mut self, value: &i16);
    fn axis_button_threshhold(&self) -> i16;
    fn set_axis_button_threshhold(&mut self, value: &i16);
}

impl Gamepad {
    pub fn new(sdl_context: &Sdl) -> Result<Gamepad, String> {
        let subsystem = sdl_context.game_controller()?;

        let game_controller = Gamepad {
            buttons: Buttons::default(),
            axis: [0; 6],

            subsystem,
            game_controller: None,

            deadzone_stick: 10_000,
            deadzone_trigger: 1_000,
            axis_button_threshhold: i16::MAX / 2,
        };

        Ok(game_controller)
    }

    pub fn update_state(&mut self) {
        if let Some(game_controller) = &self.game_controller {
            if game_controller.attached() {
                compute_state(self);
                return;
            } else {
                self.game_controller = None;
            }
        }

        reset_state(self);

        match open_game_controller(self) {
            Ok(game_controller) => self.game_controller = game_controller,
            Err(error) => ris_log::error!("{}", error),
        }
    }
}

impl IGamepad for Gamepad {
    fn buttons(&self) -> &Buttons {
        &self.buttons
    }
    fn axis(&self) -> &[i16; 6] {
        &self.axis
    }

    fn deadzone_stick(&self) -> i16 {
        self.deadzone_stick
    }
    fn set_deadzone_stick(&mut self, value: &i16) {
        self.deadzone_stick = *value
    }
    fn deadzone_trigger(&self) -> i16 {
        self.deadzone_trigger
    }
    fn set_deadzone_trigger(&mut self, value: &i16) {
        self.deadzone_trigger = *value
    }
    fn axis_button_threshhold(&self) -> i16 {
        self.axis_button_threshhold
    }
    fn set_axis_button_threshhold(&mut self, value: &i16) {
        self.axis_button_threshhold = *value
    }
}

fn compute_state(gamepad: &mut Gamepad) {
    let game_controller = gamepad.game_controller.as_ref().unwrap();

    let mut left_x = game_controller.axis(sdl2::controller::Axis::LeftX);
    let mut left_y = game_controller.axis(sdl2::controller::Axis::LeftY);
    let mut right_x = game_controller.axis(sdl2::controller::Axis::RightX);
    let mut right_y = game_controller.axis(sdl2::controller::Axis::RightY);
    let mut trigger_left = game_controller.axis(sdl2::controller::Axis::TriggerLeft);
    let mut trigger_right = game_controller.axis(sdl2::controller::Axis::TriggerRight);

    apply_deadzone_stick(&mut left_x, &mut left_y, gamepad.deadzone_stick);
    apply_deadzone_stick(&mut right_x, &mut right_y, gamepad.deadzone_stick);
    apply_deadzone_trigger(&mut trigger_left, gamepad.deadzone_trigger);
    apply_deadzone_trigger(&mut trigger_right, gamepad.deadzone_trigger);

    apply_axis_filter();

    let mut new_state = get_button_state(game_controller);

    apply_axis_as_button(
        gamepad,
        &left_x,
        sdl2::controller::Axis::LeftX,
        &mut new_state,
    );
    apply_axis_as_button(
        gamepad,
        &left_y,
        sdl2::controller::Axis::LeftY,
        &mut new_state,
    );
    apply_axis_as_button(
        gamepad,
        &right_x,
        sdl2::controller::Axis::RightX,
        &mut new_state,
    );
    apply_axis_as_button(
        gamepad,
        &right_y,
        sdl2::controller::Axis::RightY,
        &mut new_state,
    );
    apply_axis_as_button(
        gamepad,
        &trigger_left,
        sdl2::controller::Axis::TriggerLeft,
        &mut new_state,
    );
    apply_axis_as_button(
        gamepad,
        &trigger_right,
        sdl2::controller::Axis::TriggerRight,
        &mut new_state,
    );

    gamepad.buttons.update(&new_state);
    gamepad.axis[0] = left_x;
    gamepad.axis[1] = left_y;
    gamepad.axis[2] = right_x;
    gamepad.axis[3] = right_y;
    gamepad.axis[4] = trigger_left;
    gamepad.axis[5] = trigger_right;
}

fn reset_state(gamepad: &mut Gamepad) {
    gamepad.buttons.update(&0);
    gamepad.axis[0] = 0;
    gamepad.axis[1] = 0;
    gamepad.axis[2] = 0;
    gamepad.axis[3] = 0;
    gamepad.axis[4] = 0;
    gamepad.axis[5] = 0;
}

fn apply_deadzone_stick(axis_x: &mut i16, axis_y: &mut i16, deadzone: i16) {
    if *axis_x != i16::MIN
        && *axis_y != i16::MIN
        && i16::abs(*axis_x) < deadzone
        && i16::abs(*axis_y) < deadzone
    {
        *axis_x = 0;
        *axis_y = 0;
    }
}

fn apply_deadzone_trigger(axis: &mut i16, deadzone: i16) {
    if *axis != i16::MIN && i16::abs(*axis) < deadzone {
        *axis = 0;
    }
}

fn apply_axis_filter() {
    // ...
}

fn get_button_state(game_controller: &GameController) -> u32 {
    let mut new_state = 0;
    for (i, button) in ALL_BUTTONS.iter().enumerate() {
        if game_controller.button(*button) {
            new_state |= 1 << i;
        }
    }

    new_state
}

fn apply_axis_as_button(
    gamepad: &Gamepad,
    axis_value: &i16,
    axis: sdl2::controller::Axis,
    state: &mut u32,
) {
    if *axis_value < -gamepad.axis_button_threshhold {
        *state |= 1 << get_button_index(axis, true);
    } else if *axis_value > gamepad.axis_button_threshhold {
        *state |= 1 << get_button_index(axis, false);
    }
}

fn open_game_controller(gamepad: &Gamepad) -> Result<Option<GameController>, String> {
    let num_joysticks = gamepad.subsystem.num_joysticks()?;

    for index in 0..num_joysticks {
        if !gamepad.subsystem.is_game_controller(index) {
            continue;
        }

        let game_controller = gamepad
            .subsystem
            .open(index)
            .map_err(|e| format!("could not open controller {}: {}", index, e))?;

        return Ok(Some(game_controller));
    }

    Ok(None)
}

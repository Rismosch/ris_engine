use sdl2::controller::GameController;
use sdl2::GameControllerSubsystem;
use sdl2_sys::SDL_Event;
use sdl2_sys::SDL_EventType;

use ris_data::input::gamepad_data::GamepadData;

use crate::gamepad_util::{get_button_index, ALL_BUTTONS};

pub struct GamepadLogic {
    subsystem: GameControllerSubsystem,
    open_controllers: Vec<GameController>,

    last_controller_event_instance_id: u32,
    current_controller: Option<usize>,
}

impl GamepadLogic {
    pub fn new(subsystem: GameControllerSubsystem) -> Self {
        Self {
            subsystem,
            open_controllers: Vec::new(),
            last_controller_event_instance_id: u32::MAX,
            current_controller: None,
        }
    }

    pub unsafe fn handle_event(&mut self, event: &SDL_Event) {
        if event.type_ == SDL_EventType::SDL_CONTROLLERAXISMOTION as u32 {
            let which = event.caxis.which as u32;
            self.update_current_controller(which);
        }

        if event.type_ == SDL_EventType::SDL_CONTROLLERBUTTONDOWN as u32 {
            let which = event.cbutton.which as u32;
            self.update_current_controller(which);
        }

        if event.type_ == SDL_EventType::SDL_CONTROLLERBUTTONUP as u32 {
            let which = event.cbutton.which as u32;
            self.update_current_controller(which);
        }

        if event.type_ == SDL_EventType::SDL_CONTROLLERDEVICEADDED as u32 {
            let which = event.cdevice.which as u32;
            self.add_controller(which);
        }
        
        if event.type_ == SDL_EventType::SDL_CONTROLLERDEVICEREMOVED as u32 {
            let which = event.cdevice.which as u32;
            self.remove_controller(which);
        }

        if event.type_ == SDL_EventType::SDL_CONTROLLERDEVICEREMAPPED as u32 {
            let which = event.cdevice.which as u32;
            ris_log::warning!("controller \"{}\" remapped", which);
        }
    }

    pub fn post_events(&mut self, gamepad_data: &mut GamepadData) {
        if let Some(controller_index) = self.current_controller {
            let controller_to_use = &self.open_controllers[controller_index];
            compute_state(gamepad_data, controller_to_use)
        } else {
            reset_state(gamepad_data)
        }
    }

    fn update_current_controller(&mut self, instance_id: u32) {
        if self.last_controller_event_instance_id == instance_id {
            return;
        }

        for (index, open_controller) in self.open_controllers.iter().enumerate() {
            if open_controller.instance_id() == instance_id {
                self.last_controller_event_instance_id = instance_id;
                self.current_controller = Some(index);
                return;
            }
        }
    }

    fn add_controller(&mut self, joystick_index: u32) {
        let game_controller = match self.subsystem.open(joystick_index) {
            Ok(game_controller) => game_controller,
            Err(e) => {
                ris_log::error!("failed to open controller: {}", e);
                return;
            }
        };

        let instance_id = game_controller.instance_id();

        self.open_controllers.push(game_controller);

        self.last_controller_event_instance_id = instance_id;
        self.current_controller = Some(self.open_controllers.len() - 1);

        ris_log::info!(
            "controller \"{}\" added, total count: {}",
            instance_id,
            self.open_controllers.len()
        );
    }

    fn remove_controller(&mut self, instance_id: u32) {
        let mut remove_at = usize::MAX;

        for (index, open_controller) in self.open_controllers.iter().enumerate() {
            if open_controller.instance_id() == instance_id {
                remove_at = index;
                break;
            }
        }

        if remove_at < self.open_controllers.len() {
            self.open_controllers.remove(remove_at);
        }

        if self.open_controllers.is_empty() {
            self.current_controller = None;
            self.last_controller_event_instance_id = u32::MAX;
        } else {
            self.current_controller = Some(self.open_controllers.len() - 1);
            self.last_controller_event_instance_id = match self.open_controllers.last() {
                Some(last_controller) => last_controller.instance_id(),
                None => unreachable!(),
            };
        }

        ris_log::info!(
            "controller \"{}\" removed, total count: {}",
            instance_id,
            self.open_controllers.len()
        );
    }
}

fn compute_state(gamepad_data: &mut GamepadData, controller: &GameController) {
    let mut left_x = controller.axis(sdl2::controller::Axis::LeftX);
    let mut left_y = controller.axis(sdl2::controller::Axis::LeftY);
    let mut right_x = controller.axis(sdl2::controller::Axis::RightX);
    let mut right_y = controller.axis(sdl2::controller::Axis::RightY);
    let mut trigger_left = controller.axis(sdl2::controller::Axis::TriggerLeft);
    let mut trigger_right = controller.axis(sdl2::controller::Axis::TriggerRight);

    apply_deadzone_stick(&mut left_x, &mut left_y, gamepad_data.deadzone_stick);
    apply_deadzone_stick(&mut right_x, &mut right_y, gamepad_data.deadzone_stick);
    apply_deadzone_trigger(&mut trigger_left, gamepad_data.deadzone_trigger);
    apply_deadzone_trigger(&mut trigger_right, gamepad_data.deadzone_trigger);

    apply_axis_filter();

    let mut new_state = get_button_state(controller);

    apply_axis_as_button(
        gamepad_data,
        &left_x,
        sdl2::controller::Axis::LeftX,
        &mut new_state,
    );
    apply_axis_as_button(
        gamepad_data,
        &left_y,
        sdl2::controller::Axis::LeftY,
        &mut new_state,
    );
    apply_axis_as_button(
        gamepad_data,
        &right_x,
        sdl2::controller::Axis::RightX,
        &mut new_state,
    );
    apply_axis_as_button(
        gamepad_data,
        &right_y,
        sdl2::controller::Axis::RightY,
        &mut new_state,
    );
    apply_axis_as_button(
        gamepad_data,
        &trigger_left,
        sdl2::controller::Axis::TriggerLeft,
        &mut new_state,
    );
    apply_axis_as_button(
        gamepad_data,
        &trigger_right,
        sdl2::controller::Axis::TriggerRight,
        &mut new_state,
    );

    gamepad_data.buttons.update(new_state);
    gamepad_data.axis[0] = left_x;
    gamepad_data.axis[1] = left_y;
    gamepad_data.axis[2] = right_x;
    gamepad_data.axis[3] = right_y;
    gamepad_data.axis[4] = trigger_left;
    gamepad_data.axis[5] = trigger_right;
}

fn reset_state(gamepad: &mut GamepadData) {
    gamepad.buttons.set(0, 0);
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
    gamepad: &GamepadData,
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

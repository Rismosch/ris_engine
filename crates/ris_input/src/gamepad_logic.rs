use ris_data::input::gamepad_data::GamepadData;
use sdl2::{controller::GameController, GameControllerSubsystem};

use crate::gamepad_util::{get_button_index, ALL_BUTTONS};

pub fn update_gamepad(
    new_gamepad_data: &mut GamepadData,
    old_gamepad_data: &GamepadData,
    mut game_controller: Option<GameController>,
    subsystem: &GameControllerSubsystem,
) -> Option<GameController> {
    if let Some(controller) = game_controller {
        if controller.attached() {
            compute_state(new_gamepad_data, old_gamepad_data, &controller);
            return Some(controller);
        } else {
            game_controller = None;
        }
    }

    reset_state(new_gamepad_data);

    match open_game_controller(subsystem) {
        Ok(controller) => game_controller = controller,
        Err(error) => ris_log::error!("{}", error),
    }

    game_controller
}

fn compute_state(
    new_gamepad_data: &mut GamepadData,
    old_gamepad_data: &GamepadData,
    controller: &GameController,
) {
    let mut left_x = controller.axis(sdl2::controller::Axis::LeftX);
    let mut left_y = controller.axis(sdl2::controller::Axis::LeftY);
    let mut right_x = controller.axis(sdl2::controller::Axis::RightX);
    let mut right_y = controller.axis(sdl2::controller::Axis::RightY);
    let mut trigger_left = controller.axis(sdl2::controller::Axis::TriggerLeft);
    let mut trigger_right = controller.axis(sdl2::controller::Axis::TriggerRight);

    apply_deadzone_stick(&mut left_x, &mut left_y, new_gamepad_data.deadzone_stick);
    apply_deadzone_stick(&mut right_x, &mut right_y, new_gamepad_data.deadzone_stick);
    apply_deadzone_trigger(&mut trigger_left, new_gamepad_data.deadzone_trigger);
    apply_deadzone_trigger(&mut trigger_right, new_gamepad_data.deadzone_trigger);

    apply_axis_filter();

    let mut new_state = get_button_state(controller);

    apply_axis_as_button(
        new_gamepad_data,
        &left_x,
        sdl2::controller::Axis::LeftX,
        &mut new_state,
    );
    apply_axis_as_button(
        new_gamepad_data,
        &left_y,
        sdl2::controller::Axis::LeftY,
        &mut new_state,
    );
    apply_axis_as_button(
        new_gamepad_data,
        &right_x,
        sdl2::controller::Axis::RightX,
        &mut new_state,
    );
    apply_axis_as_button(
        new_gamepad_data,
        &right_y,
        sdl2::controller::Axis::RightY,
        &mut new_state,
    );
    apply_axis_as_button(
        new_gamepad_data,
        &trigger_left,
        sdl2::controller::Axis::TriggerLeft,
        &mut new_state,
    );
    apply_axis_as_button(
        new_gamepad_data,
        &trigger_right,
        sdl2::controller::Axis::TriggerRight,
        &mut new_state,
    );

    new_gamepad_data
        .buttons
        .update(&new_state, &old_gamepad_data.buttons.hold());
    new_gamepad_data.axis[0] = left_x;
    new_gamepad_data.axis[1] = left_y;
    new_gamepad_data.axis[2] = right_x;
    new_gamepad_data.axis[3] = right_y;
    new_gamepad_data.axis[4] = trigger_left;
    new_gamepad_data.axis[5] = trigger_right;
}

fn reset_state(gamepad: &mut GamepadData) {
    gamepad.buttons.update(&0, &0);
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

fn open_game_controller(
    subsystem: &GameControllerSubsystem,
) -> Result<Option<GameController>, String> {
    let num_joysticks = subsystem.num_joysticks()?;

    for index in 0..num_joysticks {
        if !subsystem.is_game_controller(index) {
            continue;
        }

        let game_controller = subsystem
            .open(index)
            .map_err(|e| format!("could not open controller {}: {}", index, e))?;

        return Ok(Some(game_controller));
    }

    Ok(None)
}

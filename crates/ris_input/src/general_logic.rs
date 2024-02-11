use std::sync::Arc;

use ris_data::god_state::GodState;
use ris_data::input::buttons::Buttons;
use ris_data::input::rebind_matrix::RebindMatrix;

pub fn update_general(state: Arc<GodState>) {
    let new_state = {
        let input = state.front.input.borrow();

        let rebound_mouse = rebind(&input.mouse.buttons, &input.mouse.rebind_matrix);
        let rebound_keyboard = rebind(&input.keyboard.buttons, &input.keyboard.rebind_matrix);
        let rebound_gamepad = rebind(&input.gamepad.buttons, &input.gamepad.rebind_matrix);

        rebound_mouse | rebound_keyboard | rebound_gamepad
    };

    state
        .front
        .input
        .borrow_mut()
        .general
        .buttons
        .update(new_state);
}

fn rebind(buttons: &Buttons, rebind_matrix: &RebindMatrix) -> u32 {
    let mut result = 0;
    let mut bitset = buttons.hold();

    while bitset != 0 {
        let bit = bitset & (!bitset + 1);
        let index = bit.trailing_zeros() as usize;

        let mask = rebind_matrix.data[index];
        result |= mask;

        bitset ^= bit;
    }

    result
}

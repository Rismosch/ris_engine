use ris_data::input::{general_data::GeneralData, buttons::Buttons, rebind_matrix::RebindMatrix};



pub fn update_general(
    new_general_data: &mut GeneralData,
    old_general_data: &GeneralData,
    mouse: &Buttons,
    keyboard: &Buttons,
    gamepad: &Buttons,
    rebind_matrix_mouse: &RebindMatrix,
    rebind_matrix_keyboard: &RebindMatrix,
    rebind_matrix_gamepad: &RebindMatrix)
{
    let rebound_mouse = rebind(mouse, rebind_matrix_mouse);
    let rebound_keyboard = rebind(keyboard, rebind_matrix_keyboard);
    let rebound_gamepad = rebind(gamepad, rebind_matrix_gamepad);

    let new_state = rebound_mouse | rebound_keyboard | rebound_gamepad;
    let old_state = old_general_data.buttons.hold();

    new_general_data.buttons.update(&new_state, &old_state);
}

fn rebind(buttons: &Buttons, rebind_matrix: &RebindMatrix) -> u32 {
    let mut result = 0;
    let mut bitset = buttons.hold();

    while bitset != 0 {
        let bit = bitset & (!bitset + 1);
        let index = bit.trailing_zeros() as usize;

        let mask = rebind_matrix[index];
        result |= mask;

        bitset ^= bit;
    }

    result
}

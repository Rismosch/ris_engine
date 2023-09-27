use ris_data::input::buttons::Buttons;
use ris_data::input::general_data::GeneralData;
use ris_data::input::rebind_matrix::RebindMatrix;

pub struct GeneralLogicArgs<'a> {
    pub new_general_data: &'a mut GeneralData,
    pub old_general_data: &'a GeneralData,
    pub mouse: &'a Buttons,
    pub keyboard: &'a Buttons,
    pub gamepad: &'a Buttons,
    pub rebind_matrix_mouse: &'a RebindMatrix,
    pub rebind_matrix_keyboard: &'a RebindMatrix,
    pub rebind_matrix_gamepad: &'a RebindMatrix,
}

pub fn update_general(args: GeneralLogicArgs) {
    let rebound_mouse = rebind(args.mouse, args.rebind_matrix_mouse);
    let rebound_keyboard = rebind(args.keyboard, args.rebind_matrix_keyboard);
    let rebound_gamepad = rebind(args.gamepad, args.rebind_matrix_gamepad);

    let new_state = rebound_mouse | rebound_keyboard | rebound_gamepad;
    let old_state = args.old_general_data.buttons.hold();

    args.new_general_data.buttons.set(new_state, old_state);
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

// use crate::gate::Gate;
// use crate::{rebind, util};
// use ris_sdl::event_pump;
// use sdl2::keyboard::Scancode;
// use std::borrow::Borrow;
// use std::collections::HashMap;

// // pub type RebindMatrix = HashMap<Scancode, HashMap<Scancode, bool>>;
// pub type Gates = HashMap<Scancode, Box<Gate>>;

// // pub static mut REBIND_MATRIX: Option<RebindMatrix> = None;
// pub static mut STATE: Option<Gates> = None;
// pub static mut STATE_REBIND: Option<Gates> = None;

// /// # Safety
// /// Should only be called by the main thread.
// /// This method modifies global static variables, and thus is inherently unsafe.
// pub unsafe fn init() {
//     let mut state = HashMap::new();
//     let mut state_rebind = HashMap::new();

//     for scancode in util::ALL_SCANCODES {
//         state.insert(scancode, Box::new(Gate::default()));
//         state_rebind.insert(scancode, Box::new(Gate::default()));
//     }

//     STATE = Some(state);
//     STATE_REBIND = Some(state_rebind);
// }

// pub fn update() {
//     let rebind_matrix = rebind::get_rebind_matrix();

//     for gate in get_state_rebind().values_mut() {
//         gate.set(false, false, false);
//     }

//     for (scancode, value) in event_pump::keyboard_state().scancodes() {
//         let gate = get_state().get_mut(&scancode).unwrap();
//         gate.update(value);

//         let rebind_row = rebind_matrix.keyboard_to_keyboard[&scancode].borrow();
//         for (rebind_scancode, is_routed) in rebind_row {
//             if !is_routed {
//                 continue;
//             }

//             let rebind_gate = get_state_rebind().get_mut(rebind_scancode).unwrap();
//             let new_up = rebind_gate.up() || gate.up();
//             let new_down = rebind_gate.down() || gate.down();
//             let new_hold = rebind_gate.hold() || gate.hold();

//             rebind_gate.set(new_up, new_down, new_hold);
//         }
//     }
// }

// pub fn up(scancode: sdl2::keyboard::Scancode) -> bool {
//     get_state_rebind()[&scancode].up()
// }

// pub fn down(scancode: sdl2::keyboard::Scancode) -> bool {
//     get_state_rebind()[&scancode].down()
// }

// pub fn hold(scancode: sdl2::keyboard::Scancode) -> bool {
//     get_state_rebind()[&scancode].hold()
// }

// fn get_state() -> &'static mut Gates {
//     unsafe {
//         match &mut STATE {
//             Some(gates) => gates,
//             None => panic!("keyboard is not initialized"),
//         }
//     }
// }

// fn get_state_rebind() -> &'static mut Gates {
//     unsafe {
//         match &mut STATE_REBIND {
//             Some(rebind_gates) => rebind_gates,
//             None => panic!("keyboard is not initialized"),
//         }
//     }
// }

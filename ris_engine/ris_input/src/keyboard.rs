use std::{borrow::Borrow, collections::HashMap};

use ris_sdl::event_pump;

use sdl2::keyboard::Scancode;

use crate::gate::Gate;

pub type RebindMatrix = HashMap<Scancode, HashMap<Scancode, bool>>;
pub type Gates = HashMap<Scancode, Box<Gate>>;

pub static mut REBIND_MATRIX: Option<RebindMatrix> = None;
pub static mut GATES: Option<Gates> = None;
pub static mut REBIND_GATES: Option<Gates> = None;

/// # Safety
/// Should only be called by the main thread.
/// This method modifies global static variables, and thus is inherently unsafe.
pub unsafe fn init() {
    let mut rebind_matrix = HashMap::new();
    let mut gates = HashMap::new();
    let mut rebind_gates = HashMap::new();

    let keyboard_state = event_pump::keyboard_state();

    for (y, _) in keyboard_state.scancodes() {
        let mut rebind_row = HashMap::new();
        for (x, _) in keyboard_state.scancodes() {
            let value = x == y;
            rebind_row.insert(x, value);
        }

        rebind_matrix.insert(y, rebind_row);
        gates.insert(y, Box::new(Gate::default()));
        rebind_gates.insert(y, Box::new(Gate::default()));
    }

    REBIND_MATRIX = Some(rebind_matrix);
    GATES = Some(gates);
    REBIND_GATES = Some(rebind_gates);
}

pub fn update() {
    let rebind_matrix = get_rebind_matrix();

    for gate in get_rebind_gates().values_mut() {
        gate.set(false, false, false);
    }

    for (scancode, value) in event_pump::keyboard_state().scancodes() {
        let gate = get_gates().get_mut(&scancode).unwrap();
        gate.update(value);

        let rebind_row = rebind_matrix[&scancode].borrow();
        for (rebind_scancode, is_routed) in rebind_row {
            if !is_routed {
                continue;
            }

            let rebind_gate = get_rebind_gates().get_mut(rebind_scancode).unwrap();
            let new_up = rebind_gate.up() || gate.up();
            let new_down = rebind_gate.down() || gate.down();
            let new_hold = rebind_gate.hold() || gate.hold();
            rebind_gate.set(new_up, new_down, new_hold);
        }
    }
}

pub fn up(scancode: sdl2::keyboard::Scancode) -> bool {
    get_rebind_gates()[&scancode].up()
}

pub fn down(scancode: sdl2::keyboard::Scancode) -> bool {
    get_rebind_gates()[&scancode].down()
}

pub fn hold(scancode: sdl2::keyboard::Scancode) -> bool {
    get_rebind_gates()[&scancode].hold()
}

fn get_rebind_matrix() -> &'static mut RebindMatrix {
    unsafe {
        match &mut REBIND_MATRIX {
            Some(rebind_matrix) => rebind_matrix,
            None => panic!("keyboard is not initialized"),
        }
    }
}

fn get_gates() -> &'static mut Gates {
    unsafe {
        match &mut GATES {
            Some(gates) => gates,
            None => panic!("keyboard is not initialized"),
        }
    }
}

fn get_rebind_gates() -> &'static mut Gates {
    unsafe {
        match &mut REBIND_GATES {
            Some(rebind_gates) => rebind_gates,
            None => panic!("keyboard is not initialized"),
        }
    }
}

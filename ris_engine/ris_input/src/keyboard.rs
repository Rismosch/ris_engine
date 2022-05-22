use std::collections::HashMap;

use ris_sdl::event_pump;

use crate::gate::Gate;

pub static mut KEY_GATES: Option<HashMap<sdl2::keyboard::Scancode, Box<Gate>>> = None;

/// # Safety
/// Should only be called by the main thread.
/// This method modifies global static variables, and thus is inherently unsafe.
pub unsafe fn init() {
    let mut key_gates = HashMap::new();

    for (scancode, _) in event_pump::keyboard_state().scancodes() {
        key_gates.insert(scancode, Box::new(Gate::new()));
    }

    KEY_GATES = Some(key_gates);
}

pub fn update() {
    let key_codes = get_key_codes();
    for (scancode, value) in event_pump::keyboard_state().scancodes() {
        let gate = key_codes.get_mut(&scancode).unwrap();
        gate.update(value);
    }
}

pub fn up(scancode: sdl2::keyboard::Scancode) -> bool {
    get_gate(scancode).up()
}

pub fn down(scancode: sdl2::keyboard::Scancode) -> bool {
    get_gate(scancode).down()
}

pub fn hold(scancode: sdl2::keyboard::Scancode) -> bool {
    get_gate(scancode).hold()
}

fn get_key_codes() -> &'static mut HashMap<sdl2::keyboard::Scancode, Box<Gate>> {
    unsafe {
        match &mut KEY_GATES {
            Some(key_codes) => key_codes,
            None => panic!("keyboard is not initialized"),
        }
    }
}

fn get_gate(scancode: sdl2::keyboard::Scancode) -> &'static Gate {
    let key_codes = get_key_codes();
    key_codes.get(&scancode).unwrap().as_ref()
}

use std::collections::HashMap;

use sdl2::keyboard::Scancode;

use crate::util;

pub type KeyboardToKeyboard = HashMap<Scancode, HashMap<Scancode, bool>>;
pub type KeyboardToMouse = bool;
pub type MouseToKeyboard = bool;
pub type MouseToMouse = [u32; 32];

pub struct RebindMatrix {
    pub keyboard_to_keyboard: KeyboardToKeyboard,
    pub keyboard_to_mouse: KeyboardToMouse,
    pub mouse_to_keyboard: MouseToKeyboard,
    pub mouse_to_mouse: MouseToMouse,
}

impl Default for RebindMatrix {
    fn default() -> Self {
        let mut keyboard_to_keyboard = default_keyboard_to_keyboard();
        let keyboard_to_mouse = false;
        let mouse_to_keyboard = false;
        let mut mouse_to_mouse = default_mouse_to_mouse();

        println!("DEBUGGING ONLY; DON'T FORGET TO DELETE THIS");

        let row = keyboard_to_keyboard.get_mut(&Scancode::Kp1).unwrap();
        *row.get_mut(&Scancode::Kp1).unwrap() = false;
        *row.get_mut(&Scancode::Kp2).unwrap() = true;
        *row.get_mut(&Scancode::Kp3).unwrap() = true;

        mouse_to_mouse[0] = 0b011;
        mouse_to_mouse[1] = 0b101;
        mouse_to_mouse[2] = 0b110;

        println!("DEBUGGING ONLY; DON'T FORGET TO DELETE THIS");

        RebindMatrix {
            keyboard_to_keyboard,
            keyboard_to_mouse,
            mouse_to_keyboard,
            mouse_to_mouse,
        }
    }
}

static mut REBIND_MATRIX: Option<RebindMatrix> = None;

/// # Safety
/// Should only be called by the main thread.
/// This method modifies global static variables, and thus is inherently unsafe.
pub unsafe fn init() {
    let rebind_matrix = RebindMatrix::default();

    REBIND_MATRIX = Some(rebind_matrix);
}

pub fn get_rebind_matrix_mut() -> &'static mut RebindMatrix {
    unsafe {
        match &mut REBIND_MATRIX {
            Some(rebind_matrix) => rebind_matrix,
            None => panic!("rebind is not initialized"),
        }
    }
}

pub fn get_rebind_matrix() -> &'static RebindMatrix {
    unsafe {
        match &REBIND_MATRIX {
            Some(rebind_matrix) => rebind_matrix,
            None => panic!("rebind is not initialized"),
        }
    }
}

fn default_keyboard_to_keyboard() -> KeyboardToKeyboard {
    let mut rebind_matrix = HashMap::new();

    for y in util::ALL_SCANCODES {
        let mut rebind_row = HashMap::new();
        for x in util::ALL_SCANCODES {
            let value = x == y;
            rebind_row.insert(x, value);
        }

        rebind_matrix.insert(y, rebind_row);
    }

    rebind_matrix
}

fn default_mouse_to_mouse() -> MouseToMouse {
    let mut rebind_matrix = [u32::default(); 32];

    for (y, rebind_mask) in rebind_matrix.iter_mut().enumerate() {
        *rebind_mask = 1 << y;
    }

    rebind_matrix
}

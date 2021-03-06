use crate::buttons::{Buttons, IButtons};

pub type RebindMatrix = [u32; 32];
pub enum RebindMatrixKind {
    Mouse,
    Keyboard,
    Gamepad,
}

pub struct General {
    buttons: Buttons,

    rebind_matrix_mouse: RebindMatrix,
    rebind_matrix_keyboard: RebindMatrix,
    rebind_matrix_gamepad: RebindMatrix,
}

impl Default for General {
    fn default() -> Self {
        let mut rebind_matrix = [0; 32];
        for (i, row) in rebind_matrix.iter_mut().enumerate() {
            *row = 1 << i;
        }

        General {
            buttons: Buttons::default(),
            rebind_matrix_mouse: rebind_matrix,
            rebind_matrix_keyboard: rebind_matrix,
            rebind_matrix_gamepad: rebind_matrix,
        }
    }
}

pub trait IGeneral {
    fn buttons(&self) -> &Buttons;

    fn rebind_matrix(&self, kind: RebindMatrixKind) -> &RebindMatrix;
    fn set_rebind_matrix(&mut self, kind: RebindMatrixKind, rebind_matrix: &RebindMatrix);
}

impl IGeneral for General {
    fn buttons(&self) -> &Buttons {
        &self.buttons
    }

    fn rebind_matrix(&self, kind: RebindMatrixKind) -> &RebindMatrix {
        match kind {
            RebindMatrixKind::Mouse => &self.rebind_matrix_mouse,
            RebindMatrixKind::Keyboard => &self.rebind_matrix_keyboard,
            RebindMatrixKind::Gamepad => &self.rebind_matrix_gamepad,
        }
    }

    fn set_rebind_matrix(&mut self, kind: RebindMatrixKind, rebind_matrix: &RebindMatrix) {
        fn set(source: &RebindMatrix, target: &mut RebindMatrix) {
            target[..32].copy_from_slice(&source[..32])
        }

        match kind {
            RebindMatrixKind::Mouse => set(rebind_matrix, &mut self.rebind_matrix_mouse),
            RebindMatrixKind::Keyboard => set(rebind_matrix, &mut self.rebind_matrix_keyboard),
            RebindMatrixKind::Gamepad => set(rebind_matrix, &mut self.rebind_matrix_gamepad),
        }
    }
}

impl General {
    pub fn update_state(&mut self, mouse: &Buttons, keyboard: &Buttons, gamepad: &Buttons) {
        let rebound_mouse = rebind(mouse, &self.rebind_matrix_mouse);
        let rebound_keyboard = rebind(keyboard, &self.rebind_matrix_keyboard);
        let rebound_gamepad = rebind(gamepad, &self.rebind_matrix_gamepad);

        let new_state = rebound_mouse | rebound_keyboard | rebound_gamepad;

        self.buttons.update(&new_state);
    }
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

use sdl2::keyboard::Scancode;

const KEY_STATE_SIZE: usize = Scancode::Num as usize;

pub type KeyState = [bool; KEY_STATE_SIZE];

impl Default for KeyState {
    fn default() -> Self {
        [false; KEY_STATE_SIZE]
    }
}

#[derive(Clone)]
pub struct Keys {
    prev: KeyState,
    state: KeyState,
}

impl Default for Keys {
    fn default() -> Self {
        Self {
            state: KeyState::default(),
            prev: KeyState::default(),
        }
    }
}

impl Keys {
    pub fn up(&self) -> KeyState {
        let mut result = [false; KEY_STATE_SIZE];
        for i in 0..KEY_STATE_SIZE {
            let state = self.state[i];
            let prev = self.prev[i];
            result[i] = !state && prev;
        }

        result
    }

    pub fn down(&self) -> KeyState {
        let mut result = [false; KEY_STATE_SIZE];
        for i in 0..KEY_STATE_SIZE {
            let state = self.state[i];
            let prev = self.prev[i];
            result[i] = state && !prev;
        }

        result
    }

    pub fn hold(&self) -> KeyState {
        self.state
    }

    pub fn is_up(&self, scancode: Scancode) -> bool {
        let index = scancode as usize;
        let state = self.state[index];
        let prev = self.prev[index];

        !state && prev
    }

    pub fn is_down(&self, scancode: Scancode) -> bool {
        let index = scancode as usize;
        let state = self.state[index];
        let prev = self.prev[index];

        state && !prev
    }

    pub fn is_hold(&self, scancode: Scancode) -> bool {
        let index = scancode as usize;
        
        self.state[index]
    }

    pub fn set(&mut self, new_state: KeyState, old_state: KeyState) {
        self.prev = old_state;
        self.state = new_state;
    }
}

use sdl2::keyboard::Scancode;

const PAGE_SIZE: usize = std::mem::size_of::<u32>();
const KEY_STATE_SIZE: usize = Scancode::Num as usize / PAGE_SIZE;

pub type KeyState = [u32; KEY_STATE_SIZE];

#[derive(Clone)]
pub struct Keys {
    state: KeyState,
    prev: KeyState,
}

impl Default for Keys {
    fn default() -> Self {
        Self {
            state: [0; KEY_STATE_SIZE],
            prev: [0; KEY_STATE_SIZE],
        }
    }
}

impl Keys {
    pub fn up(&self) -> KeyState {
        let mut result = [0; KEY_STATE_SIZE];
        for i in 0..KEY_STATE_SIZE {
            let state = self.state[i];
            let prev = self.prev[i];
            result[i] = !state & prev;
        }

        result
    }

    pub fn down(&self) -> KeyState {
        let mut result = [0; KEY_STATE_SIZE];
        for i in 0..KEY_STATE_SIZE {
            let state = self.state[i];
            let prev = self.prev[i];
            result[i] = state & !prev;
        }

        result
    }

    pub fn hold(&self) -> KeyState {
        self.state
    }

    pub fn is_up(&self, scancode: Scancode) -> bool {
        //let value = scancode as usize;
        //let i = value / PAGE_SIZE;
        //let mask = value - i;
        //let state = self.state[i];
        //let prev = 
        panic!()
    }
}

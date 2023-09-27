#[derive(Default, Clone)]
pub struct Buttons {
    state: u32,
    prev: u32,
}

impl Buttons {
    pub fn up(&self) -> u32 {
        !self.state & self.prev
    }

    pub fn down(&self) -> u32 {
        self.state & !self.prev
    }

    pub fn hold(&self) -> u32 {
        self.state
    }

    pub fn is_up(&self, actions: u32) -> bool {
        self.up() & actions != 0
    }

    pub fn is_down(&self, actions: u32) -> bool {
        self.down() & actions != 0
    }

    pub fn is_hold(&self, actions: u32) -> bool {
        self.hold() & actions != 0
    }

    pub fn set(&mut self, new_state: u32, old_state: u32) {
        self.state = new_state;
        self.prev = old_state;
    }

    pub fn update(&mut self, new_state: u32) {
        self.prev = self.state;
        self.state = new_state;
    }
}

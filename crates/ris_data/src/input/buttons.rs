#[derive(Default)]
pub struct Buttons {
    up: u32,
    down: u32,
    hold: u32,
}

impl Buttons {
    pub fn up(&self) -> u32 {
        self.up
    }

    pub fn down(&self) -> u32 {
        self.down
    }

    pub fn hold(&self) -> u32 {
        self.hold
    }

    pub fn update(&mut self, old_state: &u32, new_state: &u32) {
        self.up = !new_state & old_state;
        self.down = new_state & !old_state;
        self.hold = *new_state;
    }
}
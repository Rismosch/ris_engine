#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Gate {
    up: bool,
    down: bool,
    hold: bool,
}

impl Gate {
    pub fn update(&mut self, value: bool) {
        self.up = !value && self.hold;
        self.down = value && !self.hold;
        self.hold = value;
    }

    pub fn set(&mut self, up: bool, down: bool, hold: bool) {
        self.up = up;
        self.down = down;
        self.hold = hold;
    }

    pub fn up(&self) -> bool {
        self.up
    }

    pub fn down(&self) -> bool {
        self.down
    }

    pub fn hold(&self) -> bool {
        self.hold
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Gate {
    up: bool,
    down: bool,
    hold: bool,
}

impl Gate {
    pub fn new() -> Gate {
        Gate {
            up: false,
            down: false,
            hold: false,
        }
    }

    pub fn update(&mut self, value: bool) {
        self.up = !value && self.hold;
        self.down = value && !self.hold;
        self.hold = value;
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

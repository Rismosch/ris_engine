#[derive(Default)]
pub struct Buttons {
    up: u32,
    down: u32,
    hold: u32,
}

pub trait IButtons {
    fn up(&self) -> u32;
    fn down(&self) -> u32;
    fn hold(&self) -> u32;

    fn update(&mut self, new_state: u32);
}

impl IButtons for Buttons {
    fn up(&self) -> u32 {
        self.up
    }

    fn down(&self) -> u32 {
        self.down
    }

    fn hold(&self) -> u32 {
        self.hold
    }

    fn update(&mut self, new_state: u32) {
        let changed_buttons = new_state ^ self.hold;
        self.up = changed_buttons & self.hold;
        self.down = changed_buttons & !self.hold;
        self.hold = new_state;
    }
}

use sdl2::event::Event;

use crate::buttons::{Buttons, IButtons};

#[derive(Default)]
pub struct Mouse {
    buttons: Buttons,
    x: i32,
    y: i32,
    xrel: i32,
    yrel: i32,
    wheel_xrel: i32,
    wheel_yrel: i32,
}

pub trait IMouse {
    fn buttons(&self) -> &Buttons;
    fn x(&self) -> i32;
    fn y(&self) -> i32;
    fn xrel(&self) -> i32;
    fn yrel(&self) -> i32;
    fn wheel_xrel(&self) -> i32;
    fn wheel_yrel(&self) -> i32;
}

impl IMouse for Mouse {
    fn buttons(&self) -> &Buttons {
        &self.buttons
    }
    fn x(&self) -> i32 {
        self.x
    }
    fn y(&self) -> i32 {
        self.y
    }
    fn xrel(&self) -> i32 {
        self.xrel
    }
    fn yrel(&self) -> i32 {
        self.yrel
    }
    fn wheel_xrel(&self) -> i32 {
        self.wheel_xrel
    }
    fn wheel_yrel(&self) -> i32 {
        self.wheel_yrel
    }
}

impl Mouse {
    pub fn pre_update(&mut self) {
        self.xrel = 0;
        self.yrel = 0;
        self.wheel_xrel = 0;
        self.wheel_yrel = 0;
    }

    pub fn update(&mut self, event: &Event) {
        if let Event::MouseMotion {
            x, y, xrel, yrel, ..
        } = event
        {
            self.x = *x;
            self.y = *y;
            self.xrel += xrel;
            self.yrel += yrel;
        }

        if let Event::MouseWheel { x, y, .. } = event {
            self.wheel_xrel += x;
            self.wheel_yrel += y;
        }
    }

    pub fn update_state(&mut self, mouse_state: sdl2::mouse::MouseState) {
        let new_state = mouse_state.to_sdl_state();
        self.buttons.update(&new_state);
    }
}

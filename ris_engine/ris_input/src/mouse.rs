use ris_sdl::event_observer::IMouseObserver;
use sdl2::event::Event;

#[derive(Debug, Default)]
pub struct Mouse {
    buttons_up: u32,
    buttons_down: u32,
    buttons_hold: u32,
    x: i32,
    y: i32,
    xrel: i32,
    yrel: i32,
    wheel_xrel: i32,
    wheel_yrel: i32,
}

impl IMouseObserver for Mouse {
    fn pre_update(&mut self) {
        self.xrel = 0;
        self.yrel = 0;
        self.wheel_xrel = 0;
        self.wheel_yrel = 0;
    }

    fn update(&mut self, event: &Event) {
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

    fn update_state(&mut self, mouse_state: sdl2::mouse::MouseState) {
        let buttons = mouse_state.to_sdl_state();
        let changed_buttons = buttons ^ self.buttons_hold;
        self.buttons_down = changed_buttons & self.buttons_hold;
        self.buttons_up = changed_buttons & !self.buttons_hold;
        self.buttons_hold = buttons;
    }

    fn post_update(&mut self) {
        println!(
            "{:b} {:b} {:b}",
            self.buttons_up, self.buttons_down, self.buttons_hold
        );
    }
}

use sdl2::{event::Event, mouse::MouseState};

pub trait IMouseObserver {
    fn pre_update(&mut self);
    fn update(&mut self, events: &Event);
    fn update_state(&mut self, mouse_state: MouseState);
    fn post_update(&mut self);
}

pub trait IKeyboardObserver {
    fn pre_update(&mut self);
    fn update(&mut self, events: &Event);
}

pub trait IGameControllerObserver {
    fn pre_update(&mut self);
    fn update(&mut self, events: &Event);
}

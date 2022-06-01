use std::rc::Rc;

use ris_sdl::event_pump::IEventObserver;
use ris_sdl::event_pump::IEventPump;
use sdl2::event::Event;

#[derive(Default)]
pub struct Mouse {
    buttons: u32,
    x: i32,
    y: i32,
    rel_x: i32,
    rel_y: i32,
    wheel_relx: i32,
    wheel_rely: i32,
}

impl Mouse {
    pub fn new(event_pump: &mut impl IEventPump) -> Rc<Mouse> {
        let mouse = Rc::new(Mouse::default());
        let test = Rc::downgrade(&mouse);

        event_pump.subscribe(test);

        mouse
    }
}

impl IEventObserver for Mouse {
    fn pre_update(&self){
        println!("mouse")
    }

    fn update(&self, event: &Event) {

        if let Event::MouseWheel {x, y,..} = event {
            wheel_relx += x;
            wheel_rely += y;
        }
    }
}
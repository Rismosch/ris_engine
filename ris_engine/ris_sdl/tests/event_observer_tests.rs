// https://github.com/lpxxn/rust-design-pattern/blob/master/behavioral/observer.rs

use ris_sdl::event_observer::*;

use sdl2::event::Event;

#[derive(PartialEq)]
struct ConcreteObserver {
    id: i32,
}
impl IObserver for ConcreteObserver {
    fn update(&self, events: &Vec<Event>) {
        println!("Observer id:{} received event!", self.id);
    }
}

#[test]
fn should_subscribe(){
    panic!("bruh");
}
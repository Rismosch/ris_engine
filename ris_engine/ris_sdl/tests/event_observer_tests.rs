// https://github.com/lpxxn/rust-design-pattern/blob/master/behavioral/observer.rs

use ris_sdl::event_observer::*;

#[derive(PartialEq)]
struct ConcreteObserver {
    id: i32,
}
impl IObserver for ConcreteObserver {
    fn update(&self) {
        println!("Observer id:{} received event!", self.id);
    }
}

#[test]
fn should_subscribe(){
    panic!("bruh");
}
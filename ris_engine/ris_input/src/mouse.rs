// use crate::rebind;

// #[derive(Debug, Default)]
// struct MouseState {
//     up: u32,
//     down: u32,
//     hold: u32,
//     x: i32,
//     y: i32,
//     rel_x: i32,
//     rel_y: i32,
//     wheel_x: i32,
//     wheel_y: i32,
// }

// static mut STATE: Option<MouseState> = None;
// static mut STATE_REBIND: Option<MouseState> = None;

// /// # Safety
// /// Should only be called by the main thread.
// /// This method modifies global static variables, and thus is inherently unsafe.
// pub unsafe fn init() {
//     let state = MouseState::default();
//     let state_rebind = MouseState::default();

//     STATE = Some(state);
//     STATE_REBIND = Some(state_rebind);
// }

// pub fn update() {
//     handle_state();
//     handle_state_rebind();

//     // println!("{:?} {:?}", get_state(), get_state_rebind())
// }

use std::rc::Rc;
use std::rc::Weak;

use sdl2::event::Event;
use ris_sdl::event_pump::IEventPump;
use ris_sdl::event_pump::IEventObserver;

#[derive(Default)]
pub struct Mouse{
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
        let bruh = Rc::downgrade(&mouse);

        event_pump.subscribe_mouse(bruh);
        println!("mouse");

        mouse
    }
}

impl IEventObserver for Mouse{
    fn update(&self, events: &Vec<Event>){
        println!("bruh {} {:?}", events.len(), events);
    }
}

// fn handle_state() {
//     let event_state = ris_sdl::event_pump::get_event_state();
//     let event_mouse_state = ris_sdl::event_pump::mouse_state();
//     let sdl_mouse_state = event_mouse_state.to_sdl_state();
//     let state = get_state();

//     state.wheel_x = event_state.wheel_x;
//     state.wheel_y = event_state.wheel_y;

//     state.rel_x = event_mouse_state.x() - state.x;
//     state.rel_y = event_mouse_state.y() - state.y;
//     state.x = event_mouse_state.x();
//     state.y = event_mouse_state.y();

//     let changes = sdl_mouse_state ^ state.hold;
//     state.down = changes & sdl_mouse_state;
//     state.up = changes & !sdl_mouse_state;
//     state.hold = sdl_mouse_state;
// }

// fn handle_state_rebind() {
//     let rebind_matrix = rebind::get_rebind_matrix();
//     let mouse_to_mouse = &rebind_matrix.mouse_to_mouse;

//     let state = get_state();
//     let state_rebind = get_state_rebind();

//     state_rebind.wheel_x = state.wheel_x;
//     state_rebind.wheel_y = state.wheel_y;

//     state_rebind.rel_x = state.x;
//     state_rebind.rel_y = state.y;
//     state_rebind.x = state.x;
//     state_rebind.y = state.y;

//     state_rebind.up = 0;
//     state_rebind.down = 0;
//     state_rebind.hold = 0;

//     for (y, rebind_mask) in mouse_to_mouse.iter().enumerate().take(32) {
//         let up = (state.up & (1 << y)) != 0;
//         let down = (state.down & (1 << y)) != 0;
//         let hold = (state.hold & (1 << y)) != 0;

//         if up {
//             state_rebind.up |= rebind_mask;
//         }

//         if down {
//             state_rebind.down |= rebind_mask;
//         }

//         if hold {
//             state_rebind.hold |= rebind_mask;
//         }
//     }
// }

// fn get_state() -> &'static mut MouseState {
//     unsafe {
//         match &mut STATE {
//             Some(state) => state,
//             None => panic!("mouse is not initialized"),
//         }
//     }
// }

// fn get_state_rebind() -> &'static mut MouseState {
//     unsafe {
//         match &mut STATE_REBIND {
//             Some(state_rebind) => state_rebind,
//             None => panic!("mouse is not initialized"),
//         }
//     }
// }

// pub fn hold(button: u8) -> bool {
//     let state_rebind = get_state_rebind();

//     (state_rebind.hold & (1 << button)) != 0
// }

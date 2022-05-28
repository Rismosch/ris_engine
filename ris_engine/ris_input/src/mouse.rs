// # Safety
/// Should only be called by the main thread.
/// This method modifies global static variables, and thus is inherently unsafe.
pub unsafe fn init() {
    
}

pub fn update() {
    let mouse_state = ris_sdl::event_pump::mouse_state();

    // let test = mouse_state.to_sdl_state();
    // let test2 = sdl2::mouse::MouseState::from_sdl_state(test);

    // println!("{:?} {:?}", mouse_state, test2);
}

pub fn test<T>(bruh: T){

}

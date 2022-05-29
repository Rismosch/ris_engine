#[derive(Debug)]
struct MouseState {
    up: u32,
    down: u32,
    hold: u32,
    x: i32,
    y: i32,
    rel_x: i32,
    rel_y: i32,
    wheel_x: i32,
    wheel_y: i32,
}

static mut MOUSE_STATE: Option<MouseState> = None;
// static mut MOUSE

// # Safety
/// Should only be called by the main thread.
/// This method modifies global static variables, and thus is inherently unsafe.
pub unsafe fn init() {
    let mouse_state = MouseState {
        up: 0,
        down: 0,
        hold: 0,
        x: 0,
        y: 0,
        rel_x: 0,
        rel_y: 0,
        wheel_x: 0,
        wheel_y: 0,
    };

    MOUSE_STATE = Some(mouse_state);
}

pub fn update() {
    let event_state = ris_sdl::event_pump::get_event_state();
    let event_mouse_state = ris_sdl::event_pump::mouse_state();
    let sdl_mouse_state = event_mouse_state.to_sdl_state();
    let mouse_state = get_mouse_state();

    mouse_state.wheel_x = event_state.wheel_x;
    mouse_state.wheel_y = event_state.wheel_y;

    mouse_state.rel_x = event_mouse_state.x() - mouse_state.x;
    mouse_state.rel_y = event_mouse_state.y() - mouse_state.y;
    mouse_state.x = event_mouse_state.x();
    mouse_state.y = event_mouse_state.y();

    let changes = sdl_mouse_state ^ mouse_state.hold;
    mouse_state.down = changes & sdl_mouse_state;
    mouse_state.up = changes & !sdl_mouse_state;
    mouse_state.hold = sdl_mouse_state;

    println!("{:?}", mouse_state);
}

fn get_mouse_state() -> &'static mut MouseState {
    unsafe {
        match &mut MOUSE_STATE {
            Some(mouse_state) => mouse_state,
            None => panic!("mouse is not initialized"),
        }
    }
}

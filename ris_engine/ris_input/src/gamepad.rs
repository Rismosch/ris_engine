use sdl2::{controller::GameController, GameControllerSubsystem, Sdl};
use sdl2::event::Event;

pub struct Gamepad {
    subsystem: GameControllerSubsystem,
    game_controller: Option<GameController>,
}

impl Gamepad {
    pub fn new(sdl_context: &Sdl) -> Result<Gamepad, String> {
        let subsystem = sdl_context.game_controller()?;
        
        let game_controller = Gamepad {
            subsystem,
            game_controller: None,
        };
        
        Ok(game_controller)
    }
}

pub trait IGamepad {
    fn pre_update(&mut self);
    fn update(&mut self, event: &Event);
    fn update_state(&mut self);
}

impl IGamepad for Gamepad {
    fn pre_update(&mut self) {
        
        if let Some(game_controller) = &self.game_controller {
            if game_controller.attached() {
                println!("controller connected: {} and attached", self.game_controller.is_some());
                return;
            } else {
                println!("controller connected: {} NOT attached", self.game_controller.is_some());
                self.game_controller = None;
            }
        }
        
        println!("fetching new one");
        open_game_controller(self);
    }

    fn update(&mut self, event: &Event) {
        if let Event::JoyDeviceAdded { which, .. } = event {
            println!("attached {}", which);
        }
        
        if let Event::JoyDeviceRemoved { which, .. } = event {
            println!("removed {}", which);
        }
    }

    fn update_state(&mut self) {
        
    }
}

fn open_game_controller(gamepad: &mut Gamepad) {
    let num_joysticks = gamepad.subsystem.num_joysticks();
    if num_joysticks.is_err() {
        return;
    }
    
    let num_joysticks = num_joysticks.unwrap();
    
    for index in 0..num_joysticks {
        if !gamepad.subsystem.is_game_controller(index) {
            continue;
        }
        
        let game_controller = gamepad.subsystem.open(index);
        
        if game_controller.is_err() {
            continue;
        }
        
        let game_controller = game_controller.unwrap();
        gamepad.game_controller = Some(game_controller);

        println!("opened {} of available {}", index, num_joysticks);

        break;
    }
}

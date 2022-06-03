use sdl2::{controller::GameController, GameControllerSubsystem, Sdl};
use sdl2::event::Event;

pub struct Gamepad {
    subsystem: GameControllerSubsystem,
}

impl Gamepad {
    pub fn new(sdl_context: &Sdl) -> Result<Gamepad, String> {
        let subsystem = sdl_context.game_controller()?;

        let game_controller = Gamepad {
            subsystem,
        };

        Ok(game_controller)
    }
}

pub trait IGamepad {
    fn update(&mut self, event: &Event);
    fn update_state(&mut self);
}

impl IGamepad for Gamepad {
    fn update(&mut self, event: &Event) {
        // if let Event::JoyDeviceAdded { which, .. } = event {
        //     println!("attached {}", which);
        // }
        
        // if let Event::JoyDeviceRemoved { which, .. } = event {
        //     println!("removed {}", which);
        // }
    }

    fn update_state(&mut self) {
        let result = open_game_controller(self);
        match result {
            Ok(option) => {
                if let Some(game_controller) = option {
                    println!("{}", game_controller.attached());
                }
                else {
                    println!("bruh");
                }
            },
            Err(error) => println!("{}", error),
        }
    }
}

fn open_game_controller(gamepad: &mut Gamepad) -> Result<Option<GameController>, String> {
    let num_joysticks = gamepad.subsystem.num_joysticks()?;

    for index in 0..num_joysticks {
        if !gamepad.subsystem.is_game_controller(index) {
            continue;
        }

        let game_controller = gamepad.subsystem.open(index)
            .map_err(|e| format!("couldn't open game controller: {}", e))?;
        
        return Ok(None);
    }

    Ok(None)
}

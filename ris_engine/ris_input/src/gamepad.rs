use sdl2::{controller::GameController, GameControllerSubsystem, Sdl};
use sdl2::event::Event;

pub struct Gamepad {
    subsystem: GameControllerSubsystem,
    index: i32,
    attached: bool,
}

impl Gamepad {
    pub fn new(sdl_context: &Sdl) -> Result<Gamepad, String> {
        let subsystem = sdl_context.game_controller()?;

        let game_controller = Gamepad {
            subsystem,
            index: -1,
            attached: false,
        };

        Ok(game_controller)
    }
}

pub trait IGamepad {
    fn update(&mut self, event: &Event);
    fn update_state(&mut self) -> Result<(), String>;
}

impl IGamepad for Gamepad {
    fn update(&mut self, event: &Event) {
        if let Event::JoyDeviceAdded { which, .. } = event {
            self.index += 1;
            println!("attached {}", which);
        }
        
        if let Event::JoyDeviceRemoved { which, .. } = event {
            self.attached = false;
            println!("removed {}", which);
        }
    }

    fn update_state(&mut self) -> Result<(), String> {
        println!("{} {:?}",self.index, self.subsystem.num_joysticks());
        
        if self.attached
        {
            return Ok(());
        }

        let num_joysticks = self.subsystem.num_joysticks()?;

        // for index in 0..num_joysticks {
            // if !self.subsystem.is_game_controller(index) {
            //     continue;
            // }

            // self.subsystem.add_mapping()

            let game_controller = self.subsystem.open(0)
                .map_err(|e| format!("couldn't open game controller: {}", e));

            match game_controller {
                Ok(game_controller) => {
                    
                    self.attached = true;

                    println!("hooza {}", game_controller.instance_id());
                    
                    return Ok(());
                },
                Err(error) => println!("{}", error),
            }
            
            // self.index = index as i32;
        // }

        Ok(())
    }
}
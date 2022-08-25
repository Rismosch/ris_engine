use ris_data::info::package_info::PackageInfo;
use ris_log::log_message::LogMessage;

use crate::{
    gameloop::{run_one_frame, GameloopState},
    god_object::GodObject,
};

pub struct Engine {
    god_object: GodObject,
}

impl Engine {
    pub fn new(package_info: PackageInfo) -> Result<Engine, String> {
        let god_object = GodObject::new(package_info)?;

        let app_info = format!("{}", god_object.app_info);
        ris_log::log::forward_to_appenders(LogMessage::Plain(app_info));

        Ok(Engine { god_object })
    }

    pub fn run(&mut self) -> Result<(), String> {
        loop {
            let gameloop_state = run_one_frame(&mut self.god_object);

            match gameloop_state {
                GameloopState::Running => continue,
                GameloopState::WantsToQuit => break,
                GameloopState::Error(error) => return Err(error),
            }
        }

        Ok(())
    }
}

impl Drop for Engine {
    fn drop(&mut self) {
        ris_log::info!("engine was dropped");
    }
}

use ris_data::info::ipackage_info::IPackageInfo;

use crate::{
    bootstrapper::{bootstrap, GlobalContainer},
    gameloop::{run_one_frame, GameloopState},
};

pub struct Engine<TPackageInfo: IPackageInfo> {
    global_container: GlobalContainer<TPackageInfo>,
}

impl<TPackageInfo: IPackageInfo + std::fmt::Display> Engine<TPackageInfo> {
    pub fn new() -> Result<Engine<TPackageInfo>, String> {
        let global_container = bootstrap()?;

        ris_log::forward_to_appenders!("{}", global_container.app_info);

        Ok(Engine { global_container })
    }

    pub fn run(&mut self) -> Result<(), String> {
        loop {
            let gameloop_state = run_one_frame(&mut self.global_container);

            match gameloop_state {
                GameloopState::Running => continue,
                GameloopState::WantsToQuit => break,
                GameloopState::Error(error) => return Err(error),
            }
        }

        Ok(())
    }
}
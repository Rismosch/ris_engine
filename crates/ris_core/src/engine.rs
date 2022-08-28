use std::cell::RefCell;

use ris_data::info::package_info::PackageInfo;
use ris_jobs::{job_system::{init_run_and_block, self}, job::Job};
use ris_log::log_message::LogMessage;

use crate::{
    gameloop::{run_one_frame, GameloopState},
    god_object::GodObject,
};

pub struct Engine {
    god_object: Option<GodObject>,
}

impl Engine {
    pub fn new(package_info: PackageInfo) -> Result<Engine, String> {
        let god_object = GodObject::new(package_info)?;

        let app_info = format!("{}", god_object.app_info);
        ris_log::log::forward_to_appenders(LogMessage::Plain(app_info));

        Ok(Engine { god_object: Some(god_object) })
    }

    pub fn run(&mut self) -> Result<(), String> {
        match self.god_object.take() {
            None => Err(String::from("god_object was already taken")),
            Some(mut god_object) => {

                let result = RefCell::new(Ok(()));
                let god_result = result.clone();

                let god_job = Job::new(move || {
                    loop {
                        let gameloop_state = run_one_frame(&mut god_object);

                        for i in 0..10 {
                            job_system::submit(move || {
                                let thread_index = job_system::thread_index();
                                ris_log::debug!("{} {}", thread_index, i);
                            });
                        }

                        match gameloop_state {
                            GameloopState::Running => continue,
                            GameloopState::WantsToQuit => break,
                            GameloopState::Error(error) => {
                                *god_result.borrow_mut() = Err(error);
                                break;
                            },
                        }
                    }
                });

                job_system::init_run_and_block(god_job, 1024);
        
                let result = result.borrow_mut().to_owned();

                result
            },
        }
    }
}

impl Drop for Engine {
    fn drop(&mut self) {
        ris_log::info!("engine was dropped");
    }
}

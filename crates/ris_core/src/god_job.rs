use std::{thread, time::Duration};

use ris_data::gameloop::gameloop_state::GameloopState;
use ris_jobs::job_system;

use crate::{
    god_object::GodObject, gameloop::{output_frame::run_output, logic_frame::run_logic},
};

pub fn run(god_object: &mut GodObject) -> Result<(), String> {
    start_up(god_object);
    main_loop(god_object)
}

fn start_up(god_object: &mut GodObject) {

}

fn main_loop(god_object: &mut GodObject) -> Result<(), String> {
    // loop {
    //     let output_future = job_system::submit(|| run_output());

    //     let logic_future = job_system::submit(|| run_logic());

    //     // input

    //     job_system::wait(output_future);
    //     let gameloop_state = job_system::wait(logic_future);

    //     match gameloop_state {
    //         GameloopState::WantsToContinue => continue,
    //         GameloopState::WantsToQuit => break,
    //         GameloopState::Error(error) => return Err(error),
    //     }
    // }

    Ok(())
}
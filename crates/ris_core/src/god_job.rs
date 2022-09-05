use std::{thread, time::{Duration, Instant}, cell::{RefCell, Cell}, sync::Arc};

use ris_data::gameloop::{gameloop_state::GameloopState, input_data::{InputData, self}, logic_data::LogicData, output_data::OutputData, frame_data::{FrameData, self}};
use ris_jobs::{job_system, job_future::JobFuture};

use crate::{
    god_object::GodObject, gameloop::{input_frame, logic_frame},
};

type Frame<T> = Arc<Cell<T>>;

fn make<T>(value: T) -> Frame<T> {
    Arc::new(Cell::new(value))
}

pub fn run(god_object: &mut GodObject) -> Result<(), String> {

    let frame = make(FrameData::new());

    let current_input = make(InputData::new());
    let previous_input = make(InputData::new());

    let current_logic = make(LogicData::new());
    let previous_logic = make(LogicData::new());

    let current_output = make(OutputData::new());
    let previous_output = make(OutputData::new());

    loop {
        run_frame(frame.clone());

        // let output_future = job_system::submit(| run_output(
        //     current_output_copy.borrow_mut(),
        //     previous_output_copy.borrow(),
        //     frame
        // ));

        let logic_future = run_logic(current_logic.clone(), previous_logic.clone(), frame.clone());

        run_input(current_input.clone(), previous_input.clone(), frame.clone());

        // job_system::wait(output_future);
        let gameloop_state = job_system::wait(logic_future);

        match gameloop_state {
            GameloopState::WantsToContinue => continue,
            GameloopState::WantsToQuit => break,
            GameloopState::Error(error) => return Err(error),
        }
    }

    Ok(())
}

fn run_frame(frame: Frame<FrameData>) {
    let mut frame_data = frame.get();
    frame_data.bump();
    frame.set(frame_data);
}

fn run_output() {

}

fn run_logic(current_logic: Frame<LogicData>, previous_logic: Frame<LogicData>, frame: Frame<FrameData>) -> JobFuture<GameloopState> {
    let logic_future = job_system::submit(move || {
        let mut current_logic_data = current_logic.get();
        let previous_logic_data = previous_logic.get();
        let frame_data = frame.get();

        let result = logic_frame::run(&mut current_logic_data, &previous_logic_data, &frame_data);

        current_logic.set(current_logic_data);

        result
    });

    logic_future
}

fn run_input(current_input: Frame<InputData>, previous_input: Frame<InputData>, frame: Frame<FrameData>){
    current_input.swap(&previous_input);
    
    let mut current_input_data = current_input.get();
    let previous_input_data = previous_input.get();
    let frame_data = frame.get();

    input_frame::run(&mut current_input_data, &previous_input_data, &frame_data);

    current_input.set(current_input_data);
}
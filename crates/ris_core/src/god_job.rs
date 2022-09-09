use std::sync::Arc;

use ris_data::gameloop::{
    frame_data::FrameData, gameloop_state::GameloopState, input_data::InputData,
    logic_data::LogicData, output_data::OutputData,
};
use ris_jobs::{job_cell::JobCell, job_future::JobFuture, job_system};
use sdl2::EventPump;

use crate::{
    gameloop::{input_frame, logic_frame, output_frame},
    god_object::GodObject,
};

type Frame<T> = Arc<JobCell<T>>;

fn make<T>(value: T) -> Frame<T> {
    Arc::new(JobCell::new(value))
}

pub fn run(god_object: &mut GodObject) -> Result<(), String> {
    let frame = make(FrameData::default());

    let current_input = make(InputData::default());
    let previous_input = make(InputData::default());

    let current_logic = make(LogicData::default());
    let previous_logic = make(LogicData::default());

    let current_output = make(OutputData::default());
    let previous_output = make(OutputData::default());

    loop {
        run_frame(frame.clone());

        current_input.swap(previous_input.get());
        current_logic.swap(previous_logic.get());
        current_output.swap(previous_output.get());

        let output_future = run_output(
            current_output.clone(),
            previous_output.clone(),
            previous_logic.clone(),
            frame.clone(),
        );
        let logic_future = run_logic(
            current_logic.clone(),
            previous_logic.clone(),
            previous_input.clone(),
            frame.clone(),
        );

        let input_state = run_input(
            current_input.clone(),
            previous_input.clone(),
            frame.clone(),
            &mut god_object.event_pump,
        );

        let logic_state = job_system::wait(logic_future);
        let output_state = job_system::wait(output_future);

        match evaluate_states(input_state, logic_state, output_state) {
            GameloopState::WantsToContinue => continue,
            GameloopState::WantsToQuit => break,
            GameloopState::Error(error) => return Err(error),
        }
    }

    Ok(())
}

fn run_frame(frame: Frame<FrameData>) {
    let frame_data = frame.get();
    frame_data.bump();
    frame.swap(frame_data);
}

fn run_output(
    current_output: Frame<OutputData>,
    previous_output: Frame<OutputData>,
    previous_logic: Frame<LogicData>,
    frame: Frame<FrameData>,
) -> JobFuture<GameloopState> {
    job_system::submit(move || {
        let current_output_data = current_output.get();
        let previous_output_data = previous_output.get();
        let previous_logic_data = previous_logic.get();
        let frame_data = frame.get();

        let state = output_frame::run(
            current_output_data,
            previous_output_data,
            previous_logic_data,
            frame_data,
        );

        current_output.swap(current_output_data);

        state
    })
}

fn run_logic(
    current_logic: Frame<LogicData>,
    previous_logic: Frame<LogicData>,
    previous_input: Frame<InputData>,
    frame: Frame<FrameData>,
) -> JobFuture<GameloopState> {
    job_system::submit(move || {
        let current_logic_data = current_logic.get();
        let previous_logic_data = previous_logic.get();
        let previous_input_data = previous_input.get();
        let frame_data = frame.get();

        let state = logic_frame::run(
            current_logic_data,
            previous_logic_data,
            previous_input_data,
            frame_data,
        );

        current_logic.swap(current_logic_data);

        state
    })
}

fn run_input(
    current_input: Frame<InputData>,
    previous_input: Frame<InputData>,
    frame: Frame<FrameData>,
    event_pump: &mut EventPump,
) -> GameloopState {
    let current_input_data = current_input.get();
    let previous_input_data = previous_input.get();
    let frame_data = frame.get();

    let state = input_frame::run(
        current_input_data,
        previous_input_data,
        frame_data,
        event_pump,
    );

    current_input.swap(current_input_data);

    state
}

fn evaluate_states(
    input_state: GameloopState,
    logic_state: GameloopState,
    output_state: GameloopState,
) -> GameloopState {
    if matches!(input_state, GameloopState::WantsToContinue)
        && matches!(logic_state, GameloopState::WantsToContinue)
        && matches!(output_state, GameloopState::WantsToContinue)
    {
        return GameloopState::WantsToContinue;
    }

    if let GameloopState::Error(error) = input_state {
        return GameloopState::Error(error);
    }

    if let GameloopState::Error(error) = logic_state {
        return GameloopState::Error(error);
    }

    if let GameloopState::Error(error) = output_state {
        return GameloopState::Error(error);
    }

    GameloopState::WantsToQuit
}

use std::{cell::UnsafeCell, sync::Arc};

use ris_data::gameloop::{
    frame_data::FrameData, gameloop_state::GameloopState, input_data::InputData,
    logic_data::LogicData, output_data::OutputData,
};
use ris_jobs::{job_future::JobFuture, job_system};
use sdl2::EventPump;

use crate::{
    gameloop::{input_frame, logic_frame, output_frame},
    god_object::GodObject,
};

type Frame<T> = Arc<UnsafeCell<T>>;

pub fn run(god_object: &mut GodObject) -> Result<(), String> {
    let frame = make_frame::<FrameData>();

    let current_input = make_frame::<InputData>();
    let previous_input = make_frame::<InputData>();

    let current_logic = make_frame::<LogicData>();
    let previous_logic = make_frame::<LogicData>();

    let current_output = make_frame::<OutputData>();
    let previous_output = make_frame::<OutputData>();

    loop {
        run_frame(frame.clone());

        swap_frame_data(&current_input, get_frame_data(&previous_input));
        swap_frame_data(&current_logic, get_frame_data(&previous_logic));
        swap_frame_data(&current_output, get_frame_data(&previous_output));

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
    let frame_data = get_frame_data(&frame);
    frame_data.bump();
    swap_frame_data(&frame, frame_data);
}

fn run_output(
    current_output: Frame<OutputData>,
    previous_output: Frame<OutputData>,
    previous_logic: Frame<LogicData>,
    frame: Frame<FrameData>,
) -> JobFuture<GameloopState> {
    job_system::submit(move || {
        let current_output_data = get_frame_data(&current_output);
        let previous_output_data = get_frame_data(&previous_output);
        let previous_logic_data = get_frame_data(&previous_logic);
        let frame_data = get_frame_data(&frame);

        let state = output_frame::run(
            current_output_data,
            previous_output_data,
            previous_logic_data,
            frame_data,
        );

        swap_frame_data(&current_output, current_output_data);

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
        let current_logic_data = get_frame_data(&current_logic);
        let previous_logic_data = get_frame_data(&previous_logic);
        let previous_input_data = get_frame_data(&previous_input);
        let frame_data = get_frame_data(&frame);

        let state = logic_frame::run(
            current_logic_data,
            previous_logic_data,
            previous_input_data,
            frame_data,
        );

        swap_frame_data(&current_logic, current_logic_data);

        state
    })
}

fn run_input(
    current_input: Frame<InputData>,
    previous_input: Frame<InputData>,
    frame: Frame<FrameData>,
    event_pump: &mut EventPump,
) -> GameloopState {
    let current_input_data = get_frame_data(&current_input);
    let previous_input_data = get_frame_data(&previous_input);
    let frame_data = get_frame_data(&frame);

    let state = input_frame::run(
        current_input_data,
        previous_input_data,
        frame_data,
        event_pump,
    );

    swap_frame_data(&current_input, current_input_data);

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

fn make_frame<T: Default>() -> Frame<T> {
    Arc::new(UnsafeCell::new(T::default()))
}

#[allow(clippy::mut_from_ref)]
fn get_frame_data<T>(frame: &UnsafeCell<T>) -> &mut T {
    unsafe { &mut *frame.get() }
}

fn swap_frame_data<T>(frame: &UnsafeCell<T>, value: &mut T) {
    std::mem::swap(get_frame_data(frame), value);
}

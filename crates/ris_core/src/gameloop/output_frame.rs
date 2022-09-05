use std::cell::{RefMut, Ref};

use ris_data::gameloop::{input_data::InputData, frame_data::FrameData, logic_data::LogicData, output_data::OutputData};

pub fn run_output(
    current: RefMut<OutputData>,
    previous: Ref<OutputData>,
    frame: Ref<FrameData>)
{
    
}
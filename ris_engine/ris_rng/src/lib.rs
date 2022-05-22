pub mod rng;

mod pcg;

pub unsafe fn init() -> Result<(), Box<dyn std::error::Error>>{
    rng::init()
}
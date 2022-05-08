use crate::pcg::PCG32;

static mut PCG: PCG32 = PCG32{state: 0, increment: 0};

pub unsafe fn init(seed: [u8; 16]) {
    PCG = PCG32::seed(seed);
}

pub fn next() -> u32 {
    unsafe {
        PCG.next()
    }
}

pub fn next_f() -> f32 {
    unsafe {
        PCG.next() as f32 / 4_294_967_296.
    }
}
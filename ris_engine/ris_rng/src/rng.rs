use crate::pcg::PCG32;

static mut PCG: PCG32 = PCG32 {
    state: 0,
    increment: 0,
};

/// # Safety
/// Should only be called by the main thread.
/// This method modifies global static variables, and thus is inherently unsafe.
pub unsafe fn init(seed: [u8; 16]) {
    PCG = PCG32::seed(seed);
}

pub fn next() -> u32 {
    unsafe { PCG.next() }
}

pub fn next_f() -> f32 {
    unsafe { PCG.next() as f32 / 4_294_967_296. }
}

pub fn next_i(min: i32, max: i32) -> i32 {
    if max <= min {
        if max == min {
            return min;
        } else {
            return !0;
        }
    }

    let r = (((max - min + 1) as f32) * next_f()) as i32 + min;

    if r > max {
        max
    } else {
        r
    }
}

use crate::pcg::PCG32;

pub type Seed = [u8; 16];

static mut SEED: Seed = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
static mut PCG: PCG32 = PCG32 {
    state: 0,
    increment: 0,
};

/// # Safety
/// Should only be called by the main thread.
/// This method modifies global static variables, and thus is inherently unsafe.
pub unsafe fn init() -> Result<(), Box<dyn std::error::Error>> {
    let now = std::time::SystemTime::now();
    let duration_since_epoch = now.duration_since(std::time::UNIX_EPOCH)?;

    let seed: Seed = duration_since_epoch.as_millis().to_le_bytes();
    init_with_seed(seed);
    Ok(())
}

/// # Safety
/// Should only be called by the main thread.
/// This method modifies global static variables, and thus is inherently unsafe.
pub unsafe fn init_with_seed(seed: Seed) {
    SEED = seed;
    PCG = PCG32::seed(seed);
}

pub fn seed() -> Seed {
    unsafe { SEED }
}

pub fn next_u() -> u32 {
    unsafe { PCG.next() }
}

pub fn next_f() -> f32 {
    unsafe {
        // PCG.next() as f32 / 4_294_967_296.
        f32::from_bits(0x3F80_0000 | (PCG.next() & 0x7F_FFFF)) - 1.
    }
}

pub fn range_i(min: i32, max: i32) -> i32 {
    if max <= min {
        if max == min {
            return min;
        } else {
            return i32::MIN;
        }
    }

    let r = (((max - min + 1) as f32) * next_f()) as i32 + min;

    if r > max {
        max
    } else {
        r
    }
}

pub fn range_f(min: f32, max: f32) -> f32 {
    if max <= min {
        if max == min {
            return min;
        } else {
            return f32::NAN;
        }
    }

    let r = (max - min + 1.) * next_f() + min;

    if r > max {
        max
    } else {
        r
    }
}

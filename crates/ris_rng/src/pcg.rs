#[derive(Debug)]
pub struct Pcg32 {
    pub state: u64,
    pub increment: u64,
}

impl Pcg32 {
    pub fn new() -> Pcg32 {
        Pcg32 {
            state: 0xcafef00dd15ea5e5,
            increment: 0xa02bdbf7bb3c0a7,
        }
    }

    pub fn new_from_seed(seed: u128) -> Pcg32 {
        let mut result = Pcg32::new();

        let mask = 0xFFFF_FFFF_FFFF_FFFF;
        let state = seed as u64 & mask;
        let increment = (seed >> 64) as u64 & mask;

        result.state = state.wrapping_add(increment);

        result
    }

    pub fn next(&mut self) -> u32 {
        const MULTIPLIER: u64 = 6364136223846793005;
        const XSHIFT: u32 = 18;
        const SPARE: u32 = 27;
        const ROTATE: u32 = 59;

        let oldstate = self.state;
        // Advance internal state
        self.state = self
            .state
            .wrapping_mul(MULTIPLIER)
            .wrapping_add(self.increment | 1);
        // Calculate output function (XSH RR), uses old state for max ILP
        let xorshifted = (((oldstate >> XSHIFT) ^ oldstate) >> SPARE) as u32;
        let rot = (oldstate >> ROTATE) as u32;
        xorshifted.rotate_right(rot)
    }
}

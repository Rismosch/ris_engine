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

    pub fn new_from_seed(seed: [u8; 16]) -> Pcg32 {
        let mut result = Pcg32::new();

        let state = (seed[0o00] as u64)
            | ((seed[0o01] as u64) << 0o10)
            | ((seed[0o02] as u64) << 0o20)
            | ((seed[0o03] as u64) << 0o30)
            | ((seed[0o04] as u64) << 0o40)
            | ((seed[0o05] as u64) << 0o50)
            | ((seed[0o06] as u64) << 0o60)
            | ((seed[0o07] as u64) << 0o70);

        let increment = (seed[0o10] as u64)
            | ((seed[0o11] as u64) << 0o10)
            | ((seed[0o12] as u64) << 0o20)
            | ((seed[0o13] as u64) << 0o30)
            | ((seed[0o14] as u64) << 0o40)
            | ((seed[0o15] as u64) << 0o50)
            | ((seed[0o16] as u64) << 0o60)
            | ((seed[0o17] as u64) << 0o70);

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

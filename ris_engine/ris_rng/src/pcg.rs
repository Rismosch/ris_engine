pub struct PCG32 {
    pub state: u64,
    pub increment: u64,
}

impl PCG32 {
    pub fn seed(seed: [u8; 16]) -> PCG32 {
        let mut result = PCG32 {
            state: 0xcafef00dd15ea5e5,
            increment: 0xa02bdbf7bb3c0a7,
        };

        let state = (seed[0o00] as u64)
            | (seed[0o01] as u64) << 0x08
            | (seed[0o02] as u64) << 0x10
            | (seed[0o03] as u64) << 0x18
            | (seed[0o04] as u64) << 0x20
            | (seed[0o05] as u64) << 0x28
            | (seed[0o06] as u64) << 0x30
            | (seed[0o07] as u64) << 0x38;

        let increment = (seed[0o10] as u64)
            | (seed[0o11] as u64) << 0x08
            | (seed[0o12] as u64) << 0x10
            | (seed[0o13] as u64) << 0x18
            | (seed[0o14] as u64) << 0x20
            | (seed[0o15] as u64) << 0x28
            | (seed[0o16] as u64) << 0x30
            | (seed[0o17] as u64) << 0x38;

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

pub struct PCG32{
    state: u64,
    increment: u64
}

impl PCG32 {
    pub fn next(&mut self) -> u32 {
        let oldstate = self.state;
        // Advance internal state
        self.state = oldstate * 6364136223846793005ULL + (self.increment | 1);
        // Calculate output function (XSH RR), uses old state for max ILP
        let xorshifted: u32 = ((oldstate >> 18u) ^ oldstate) >> 27u;
        let rot: u32 = oldstate >> 59u;
        (xorshifted >> rot) | (xorshifted << ((-rot) & 31))
    }
}
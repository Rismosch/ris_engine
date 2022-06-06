use crate::pcg::Pcg32;

pub type Seed = [u8; 16];

pub struct Rng {
    seed: Seed,
    pcg: Pcg32,
}

impl Rng {
    pub fn new() -> Result<Rng, Box<dyn std::error::Error>> {
        let now = std::time::SystemTime::now();
        let duration_since_epoch = now.duration_since(std::time::UNIX_EPOCH)?;
        let seed: Seed = duration_since_epoch.as_millis().to_le_bytes();

        let rng = Rng::new_from_seed(seed);
        Ok(rng)
    }

    pub fn new_from_seed(seed: Seed) -> Rng {
        Rng {
            seed,
            pcg: Pcg32::new_from_seed(seed),
        }
    }

    pub fn seed(&self) -> Seed {
        self.seed
    }

    pub fn next_u(&mut self) -> u32 {
        self.pcg.next()
    }

    pub fn next_f(&mut self) -> f32 {
        f32::from_bits(0x3F80_0000 | (self.next_u() & 0x7F_FFFF)) - 1.
    }

    pub fn range_f(&mut self, min: f32, max: f32) -> f32 {
        if max <= min {
            if max == min {
                return min;
            } else {
                return f32::NAN;
            }
        }

        let r = (max - min + 1.) * self.next_f() + min;

        if r > max {
            max
        } else {
            r
        }
    }

    pub fn range_i(&mut self, min: i32, max: i32) -> i32 {
        if max <= min {
            if max == min {
                return min;
            } else {
                return i32::MIN;
            }
        }

        let r = (((max - min + 1) as f32) * self.next_f()) as i32 + min;

        if r > max {
            max
        } else {
            r
        }
    }
}

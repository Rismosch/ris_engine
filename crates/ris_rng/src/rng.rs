use ris_error::RisResult;

use crate::pcg::Pcg32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Seed(pub [u8; 16]);

impl Seed {
    #[cfg(not(miri))]
    pub fn new() -> RisResult<Self> {
        let now = std::time::SystemTime::now();
        let duration_since_epoch = now.duration_since(std::time::UNIX_EPOCH)?;
        let bytes = duration_since_epoch.as_millis().to_le_bytes();
        let seed = Seed(bytes);

        Ok(seed)
    }

    #[cfg(miri)]
    pub fn new() -> RisResult<Self> {
        Ok(Self([
            198, 237, 209, 128, 44, 192, 237, 30, 31, 198, 222, 241, 131, 161, 105, 206,
        ]))
    }
}

pub struct Rng {
    seed: Seed,
    pcg: Pcg32,
}

impl Rng {
    pub fn new(seed: Seed) -> Rng {
        let mut pcg = Pcg32::new_from_seed(seed.0);
        let _ = pcg.next();

        Rng { seed, pcg }
    }

    pub fn seed(&self) -> &Seed {
        &self.seed
    }

    pub fn next_u(&mut self) -> u32 {
        self.pcg.next()
    }

    pub fn next_bool(&mut self) -> bool {
        (self.next_u() & 1) == 1
    }

    pub fn next_byte(&mut self) -> u8 {
        (0xFF & self.next_u()) as u8
    }

    pub fn next_bytes(&mut self, buf_len: usize) -> Vec<u8> {
        let mut buf = vec![0; buf_len];
        for item in buf.iter_mut().take(buf_len) {
            *item = self.next_byte();
        }

        buf
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

    /// min and max are inclusive
    pub fn range_i(&mut self, min: i32, max: i32) -> i32 {
        let max = max + 1;
        if max <= min {
            if max == min {
                return min;
            } else {
                return i32::MIN;
            }
        }

        let r = (((max - min) as f32) * self.next_f()) as i32 + min;

        if r > max {
            max
        } else {
            r
        }
    }
}

use ris_error::RisResult;
use ris_math::quaternion::Quat;
use ris_math::vector::Vec2;
use ris_math::vector::Vec3;
use ris_math::vector::Vec4;

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

    /// returns a random u32
    pub fn next_u32(&mut self) -> u32 {
        self.pcg.next()
    }

    /// returns a random i32
    pub fn next_i32(&mut self) -> i32 {
        i32::from_ne_bytes(self.next_u32().to_ne_bytes())
    }

    /// returns a random bool
    pub fn next_bool(&mut self) -> bool {
        (self.next_u32() & 1) == 1
    }

    /// returns a random u8
    pub fn next_byte(&mut self) -> u8 {
        (0xFF & self.next_u32()) as u8
    }

    /// returns a Vec initialized with random u8s
    pub fn next_bytes(&mut self, buf_len: usize) -> Vec<u8> {
        let mut buf = vec![0; buf_len];
        for item in buf.iter_mut().take(buf_len) {
            *item = self.next_byte();
        }

        buf
    }

    /// returns a random f32 between 0.0 and 1.0
    pub fn next_f32(&mut self) -> f32 {
        f32::from_bits(0x3F80_0000 | (self.next_u32() & 0x7F_FFFF)) - 1.
    }

    /// returns a random f32 between min and max
    pub fn range_f(&mut self, min: f32, max: f32) -> f32 {
        if max <= min {
            if max == min {
                return min;
            } else {
                return f32::NAN;
            }
        }

        let r = (max - min) * self.next_f32() + min;

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

        let r = (((max - min) as f32) * self.next_f32()) as i32 + min;

        if r > max {
            max
        } else {
            r
        }
    }

    pub fn next_pos_2(&mut self) -> Vec2 {
        let x = self.range_f(-1.0, 1.0);
        let y = self.range_f(-1.0, 1.0);

        Vec2(x, y)
    }

    pub fn next_pos_3(&mut self) -> Vec3 {
        let x = self.range_f(-1.0, 1.0);
        let y = self.range_f(-1.0, 1.0);
        let z = self.range_f(-1.0, 1.0);

        Vec3(x, y, z)
    }

    pub fn next_pos_4(&mut self) -> Vec4 {
        let x = self.range_f(-1.0, 1.0);
        let y = self.range_f(-1.0, 1.0);
        let z = self.range_f(-1.0, 1.0);
        let w = self.range_f(-1.0, 1.0);

        Vec4(x, y, z, w)
    }

    pub fn next_dir_2(&mut self) -> Vec2 {
        self.next_pos_2().normalize()
    }

    pub fn next_dir_3(&mut self) -> Vec3 {
        self.next_pos_3().normalize()
    }

    pub fn next_dir_4(&mut self) -> Vec4 {
        self.next_pos_4().normalize()
    }

    pub fn next_rot(&mut self) -> Quat {
        let vec4 = self.next_dir_4();
        Quat::from(vec4)
    }
}

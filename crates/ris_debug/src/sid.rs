#[derive(Debug, Clone)]
pub struct Sid {
    pub hash: u32,
    #[cfg(debug_assertions)]
    pub value: String,
}

impl std::fmt::Display for Sid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        #[cfg(debug_assertions)]
        {
            write!(f, "{} ({})", self.value, self.hash)
        }

        #[cfg(not(debug_assertions))]
        {
            write!(f, "sid_{}", self.hash)
        }
    }
}

impl PartialEq for Sid {
    fn eq(&self, other: &Self) -> bool {
        #[cfg(debug_assertions)]
        {
            let result = self.hash == other.hash;

            let left = &self.value;
            let right = &other.value;
            let hash = self.hash;
            if result && left != right {
                ris_error::throw!("sid collision detected! left: \"{}\" right: \"{}\" hash: \"{}\". this should never happen. change one of the strings to something else", left, right, hash );
            }

            result
        }

        #[cfg(not(debug_assertions))]
        {
            self.hash == other.hash
        }
    }
}

impl Eq for Sid {}

impl std::hash::Hash for Sid {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_u32(self.hash);
    }
}

#[macro_export]
macro_rules! sid {
    ($value:expr) => {{
        const HASH: u32 = {
            let bytes = $value.as_bytes();
            let mut hash = 2166136261u32;

            // const for-loops are not supported yet, thus a crude while must do the job
            // https://github.com/rust-lang/rust/issues/87575
            let mut i = 0;
            while i < bytes.len() {
                let byte = bytes[i];
                let byte_as_int: u32 = byte as u32;
                hash ^= byte_as_int;
                hash = hash.wrapping_mul(16777619u32);
                i += 1;
            }

            hash
        };

        $crate::sid::Sid {
            hash: HASH,
            #[cfg(debug_assertions)]
            value: $value.to_string(),
        }
    }};
}

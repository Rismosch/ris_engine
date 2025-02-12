#[derive(Debug, Clone)]
pub struct Sid {
    pub hash: u32,
    #[cfg(feature = "store_sid_values")]
    pub value: String,
}

impl Sid {
    pub fn from(hash: u32, value: String) -> Self {
        #[cfg(feature = "store_sid_values")]
        {
            Self { hash, value }
        }

        #[cfg(not(feature = "store_sid_values"))]
        {
            let _ = value;

            Self { hash }
        }
    }
}

impl std::fmt::Display for Sid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        #[cfg(feature = "store_sid_values")]
        {
            write!(f, "{} ({})", self.value, self.hash)
        }

        #[cfg(not(feature = "store_sid_values"))]
        {
            write!(f, "sid_{}", self.hash)
        }
    }
}

impl PartialEq for Sid {
    fn eq(&self, other: &Self) -> bool {
        #[cfg(feature = "store_sid_values")]
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

        #[cfg(not(feature = "store_sid_values"))]
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

pub const PRIME: u32 = 2166136261u32;

#[macro_export]
macro_rules! sid {
    ($value:expr) => {{
        const HASH: u32 = {
            let bytes = $value.as_bytes();

            let mut hash = $crate::sid::PRIME;
            $crate::const_hash!(hash, bytes);

            hash
        };

        $crate::sid::Sid::from(HASH, $value.to_string())
    }};
}

#[macro_export]
macro_rules! fsid {
    () => {{
        $crate::fsid!(0u32)
    }};
    ($salt:expr) => {{
        const FILE: &str = file!();
        const LINE: u32 = line!();
        const SALT: u32 = $salt;

        const HASH: u32 = {
            let file_bytes = FILE.as_bytes();
            let line_bytes = LINE.to_le_bytes();
            let salt_bytes = SALT.to_le_bytes();

            let mut hash = $crate::sid::PRIME;
            $crate::const_hash!(hash, file_bytes);
            $crate::const_hash!(hash, line_bytes);
            $crate::const_hash!(hash, salt_bytes);

            hash
        };

        $crate::sid::Sid::from(HASH, format!("{}:{}/salt={}", FILE, LINE, SALT))
    }};
}

#[macro_export]
macro_rules! const_hash {
    ($hash:expr, $bytes:expr) => {{
        let mut i = 0;
        while i < $bytes.len() {
            let byte = $bytes[i];
            let byte_as_int: u32 = byte as u32;
            $hash ^= byte_as_int;
            $hash = $hash.wrapping_mul(16777619u32);
            i += 1;
        }
    }};
}

const MAX_DISTANCE: u32 = 0x80000000;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Counter(u32);

impl PartialOrd for Counter {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Counter {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let lhs = self.0;
        let rhs = other.0;
        let distance = lhs.abs_diff(rhs);

        if distance < MAX_DISTANCE {
            lhs.cmp(&rhs)
        } else {
            rhs.cmp(&lhs)
        }
    }
}

impl Counter {
    pub const MAX: Self = Counter(u32::MAX);

    pub fn raw(self) -> u32 {
        self.0
    }

    pub fn from_raw(value: u32) -> Self {
        Self(value)
    }

    pub fn increase(&mut self) {
        self.0 = self.0.wrapping_add(1);
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Counter(usize);

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

        const MAX_DISTANCE: usize = 0x80usize.swap_bytes();
        if distance < MAX_DISTANCE {
            lhs.cmp(&rhs)
        } else {
            rhs.cmp(&lhs)
        }
    }
}

impl Counter {
    pub fn raw(self) -> usize {
        self.0
    }

    pub fn from_raw(value: usize) -> Self {
        Self(value)
    }

    pub fn add_one(&mut self) {
        self.0 = self.0.wrapping_add(1);
    }
}


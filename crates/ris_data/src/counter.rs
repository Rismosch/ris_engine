#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Counter(usize);

impl PartialOrd for Counter {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let lhs = self.0;
        let rhs = other.0;
        let distance = lhs.abs_diff(rhs);

        const MAX_DISTANCE: usize = isize::MAX as usize;
        if distance < MAX_DISTANCE {
            lhs.partial_cmp(&rhs)
        } else {
            rhs.partial_cmp(&lhs)
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
}

impl std::ops::Add<isize> for Counter {
    type Output = Self;

    fn add(self, rhs: isize) -> Self::Output {
        let to_add = usize::from_ne_bytes(rhs.to_ne_bytes());
        let raw = self.raw();
        let new_raw = raw.wrapping_add(to_add);
        Self::from_raw(new_raw)

        //if rhs.is_positive() {
        //    let to_add = rhs as usize;
        //    let raw = self.raw();
        //    let new_raw = raw.wrapping_add(to_add);
        //    Self::from_raw(new_raw)
        //} else if rhs == isize::MIN {
        //    let to_subtract = isize::MAX as usize;
        //    let raw = self.raw();
        //    let new_raw = raw.wrapping_sub(1).wrapping_sub(to_subtract);
        //    let test = !raw;
        //    assert_eq!(new_raw, test);
        //    Self::from_raw(new_raw)
        //} else {
        //    let to_subtract = rhs.abs() as usize;
        //    let raw = self.raw();
        //    let new_raw = raw.wrapping_sub(to_subtract);
        //    Self::from_raw(new_raw)
        //}
    }
}

impl std::ops::AddAssign<isize> for Counter {
    fn add_assign(&mut self, rhs: isize) {
        *self = *self + rhs;
    }
}

impl std::ops::Sub<isize> for Counter {
    type Output = Self;

    fn sub(self, rhs: isize) -> Self::Output {
        let to_subtract = usize::from_ne_bytes(rhs.to_ne_bytes());
        let raw = self.raw();
        let new_raw = raw.wrapping_sub(to_subtract);
        Self::from_raw(new_raw)
    }
}

impl std::ops::SubAssign<isize> for Counter {
    fn sub_assign(&mut self, rhs: isize) {
        *self = *self - rhs
    }
}

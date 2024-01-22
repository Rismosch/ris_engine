#[derive(Clone)]
pub struct RebindMatrix {
    pub data: [u32; 32],
}

impl Default for RebindMatrix {
    fn default() -> Self {
        let mut result = Self::new_empty();
        for (i, row) in result.data.iter_mut().enumerate() {
            *row = 1 << i;
        }

        result
    }
}

impl RebindMatrix {
    pub fn new_empty() -> Self {
        Self { data: [0; 32] }
    }

    pub fn copy(source: &Self, target: &mut Self) {
        target.data[..32].copy_from_slice(&source.data[..32])
    }
}

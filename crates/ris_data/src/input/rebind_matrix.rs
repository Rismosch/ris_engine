#[derive(Clone)]
pub struct RebindMatrix {
    pub data: [u32; 32],
}

impl Default for RebindMatrix {
    fn default() -> Self {
        let mut data = [0; 32];
        for (i, row) in data.iter_mut().enumerate() {
            *row = 1 << i;
        }

        Self {data}
    }
}

impl RebindMatrix {
    pub fn copy(source: &Self, target: &mut Self) {
        target.data[..32].copy_from_slice(&source.data[..32])
    }
}

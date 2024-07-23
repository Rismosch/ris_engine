use ris_error::RisResult;

pub trait IFrame {
    unsafe fn free(&mut self, device: &ash::Device);
}

pub struct Frames<TFrame: IFrame> {
    index: usize,
    count: usize,
    frames: Vec<TFrame>,
}

impl<TFrame: IFrame> Frames<TFrame> {
    /// # Safety
    ///
    /// `free()` must be called, or you are leaking memory.
    pub unsafe fn alloc(
        count: usize,
        mut alloc_callback: impl FnMut(usize) -> RisResult<TFrame>,
    ) -> RisResult<Self> {
        let frames = (0..count)
            .map(|i| alloc_callback(i))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            index: 0,
            count,
            frames,
        })
    }

    /// # Safety
    ///
    /// Must only be called once. Memory must not be freed twice.
    pub unsafe fn free(&mut self, device: &ash::Device) {
        for frame in self.frames.iter_mut() {
            frame.free(device);
        }
    }

    pub fn acquire_next(&mut self) -> &mut TFrame {
        let result = &mut self.frames[self.index];
        self.index = (self.index + 1) % self.count;
        result
    }
}

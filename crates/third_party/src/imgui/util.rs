use super::ImVector;

/// Safety: `value` must be a valid `ImVector<T>`
pub unsafe fn im_vector_to_slice<T>(value: &ImVector<T>) -> &[T] {
    unsafe {
        let ptr = value.Data;
        let length = value.Size as usize;

        std::slice::from_raw_parts(ptr, length)
    }
}

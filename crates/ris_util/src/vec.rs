/// fastest way to copy a slice into a vec
pub fn fast_copy<T: Copy>(vec: &mut Vec<T>, slice: &[T]) {
    let len = slice.len();
    unsafe { vec.set_len(len) };
    vec.copy_from_slice(slice);
}

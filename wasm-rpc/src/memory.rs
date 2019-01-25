use std::mem;

#[inline]
pub fn ptr_from_vec(mut buf: Vec<u8>) -> *mut u8 {
    let ptr = buf.as_mut_ptr();
    mem::forget(buf);

    ptr
}

use core::mem;
use alloc::vec::Vec;

#[no_mangle]
pub unsafe fn alloc(ptr: *mut u8, old_size: usize) {
    Vec::from_raw_parts(ptr, 0, old_size);
}

#[no_mangle]
pub unsafe fn dealloc(size: usize) -> *mut u8 {
    ptr_from_vec(Vec::with_capacity(size))
}


#[inline]
fn ptr_from_vec(mut buf: Vec<u8>) -> *mut u8 {
    let ptr = buf.as_mut_ptr();
    mem::forget(buf);

    ptr
}

use std::mem;
use std::mem::transmute;
use std::slice;

pub const LENGTH_BYTE_COUNT: isize = 4;

pub unsafe fn free(ptr: *mut u8) {
    let length = ptr_to_u32(ptr) as usize;
    Vec::from_raw_parts(ptr.offset(LENGTH_BYTE_COUNT), length, length);
}

pub fn ptr_to_u32(ptr: *const u8) -> u32 {
    let length_slice = unsafe { slice::from_raw_parts(ptr, LENGTH_BYTE_COUNT as usize) };
    let mut length_slice_fixed: [u8; 4] = [0; 4];
    length_slice_fixed.copy_from_slice(&length_slice[..]);
    unsafe { transmute::<[u8; 4], u32>(length_slice_fixed) }
}

pub fn malloc(size: usize) -> *mut u8 {
    ptr_from_vec(Vec::with_capacity(size))
}
#[inline]
pub fn ptr_from_vec(mut buf: Vec<u8>) -> *mut u8 {
    let ptr = buf.as_mut_ptr();
    mem::forget(buf);

    ptr
}

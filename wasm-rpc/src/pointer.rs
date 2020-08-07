use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_cbor::{from_slice, to_vec};
use std::convert::TryInto;
use std::mem;
use std::mem::ManuallyDrop;
use std::slice;

pub type Pointer = *const u8;
pub const LENGTH_BYTE_COUNT: isize = 4;

pub unsafe fn free(ptr: *mut u8) {
    let length = ptr_to_u32(ptr) as usize;
    Vec::from_raw_parts(ptr.offset(LENGTH_BYTE_COUNT), length, length);
}

pub fn ptr_to_u32(ptr: *const u8) -> u32 {
    let length_slice = unsafe { slice::from_raw_parts(ptr, LENGTH_BYTE_COUNT as usize) };
    u32::from_le_bytes(length_slice.try_into().unwrap())
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

pub fn to_value<T: DeserializeOwned>(ptr: Pointer) -> Result<T, serde_cbor::Error> {
    from_slice::<T>(to_bytes(ptr))
}

pub fn to_bytes<'a>(ptr: Pointer) -> &'a [u8] {
    let length = ptr_to_u32(ptr) as usize;
    unsafe { slice::from_raw_parts(ptr.offset(LENGTH_BYTE_COUNT), length) }
}

pub fn from_value<V: Serialize>(value: &V) -> Pointer {
    from_bytes(&to_vec(&value).unwrap())
}

pub fn from_bytes(bytes: &[u8]) -> Pointer {
    let value_and_length = [&(bytes.len() as i32).to_le_bytes()[..], bytes].concat();
    ManuallyDrop::new(value_and_length).as_mut_ptr()
}

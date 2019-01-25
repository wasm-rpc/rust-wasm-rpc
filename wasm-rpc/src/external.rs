use bytes::{FromBytes, ToBytes};
use pointer::{Dereferenceable, Referenceable};
extern "C" {
    fn _get_memory(key: *const u8) -> *const u8;
    fn _set_memory(key: *const u8, value: *const u8);
    fn _get_storage(key: *const u8) -> *const u8;
    fn _set_storage(key: *const u8, value: *const u8);
}

pub fn set_memory<T: ToBytes>(key: Vec<u8>, value: T) {
    unsafe { _set_memory(key.as_pointer(), value.to_bytes().as_pointer()) }
}

pub fn get_memory<T: FromBytes>(key: Vec<u8>) -> T {
    let v: Vec<u8> = unsafe { _get_memory(key.as_pointer()) }.as_raw_bytes();
    FromBytes::from_bytes(v)
}

pub fn set_storage<T: ToBytes>(key: Vec<u8>, value: T) {
    unsafe { _set_memory(key.as_pointer(), value.to_bytes().as_pointer()) }
}

pub fn get_storage<T: FromBytes>(key: Vec<u8>) -> T {
    let v: Vec<u8> = unsafe { _get_memory(key.as_pointer()) }.as_raw_bytes();
    FromBytes::from_bytes(v)
}

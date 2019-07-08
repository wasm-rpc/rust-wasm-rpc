#![feature(
    set_stdio,
    alloc_error_handler,
    core_intrinsics,
    panic_info_message,
    try_trait
)]

extern crate serde_cbor;
mod bytes;
mod debug;
pub mod error;
mod memory;
mod pointer;
mod response;
mod set_stdio;

pub use bytes::{FromBytes, ToBytes};
pub use pointer::{Dereferenceable, Pointer, Referenceable};
pub use response::{Bytes, Responsable};
pub use serde_cbor::{from_slice, to_vec, ObjectKey, Value};
pub use std::collections::BTreeMap;

pub use set_stdio::set_stdio;

// Note these must be defined in lib.rs
// https://users.rust-lang.org/t/how-to-export-rust-functions-from-child-module/11057/7
#[no_mangle]
pub unsafe fn __free(ptr: *mut u8) {
    memory::free(ptr);
}

#[no_mangle]
pub unsafe fn __malloc(size: usize) -> *mut u8 {
    memory::malloc(size)
}

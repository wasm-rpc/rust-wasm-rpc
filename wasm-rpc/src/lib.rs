#![feature(
    alloc,
    alloc_error_handler,
    core_intrinsics,
    panic_info_message,
    try_trait
)]
#![cfg_attr(debug_assertions, feature(set_stdio))]

extern crate serde_cbor;
mod bytes;
pub mod error;
mod memory;
mod pointer;
mod response;

pub use bytes::{FromBytes, ToBytes};
use memory::ptr_from_vec;
pub use pointer::{Dereferenceable, Pointer, Referenceable};
pub use response::{Bytes, Responsable};
pub use serde_cbor::{from_slice, to_vec, ObjectKey, Value};
pub use std::collections::BTreeMap;

#[cfg(debug_assertions)]
mod hook;
#[cfg(debug_assertions)]
pub use hook::hook;
#[cfg(debug_assertions)]
pub use hook::{_eprint_args, _print_args};

// Note these must be defined in lib.rs
// https://users.rust-lang.org/t/how-to-export-rust-functions-from-child-module/11057/7
#[no_mangle]
pub unsafe fn dealloc(ptr: *mut u8, old_size: usize) {
    Vec::from_raw_parts(ptr, 0, old_size);
}
#[no_mangle]
pub unsafe fn alloc(size: usize) -> *mut u8 {
    ptr_from_vec(Vec::with_capacity(size))
}

/// Overrides the default `print!` macro.
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => (#[cfg(debug_assertions)]
        $crate::_print_args(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => (print!("\n"));
    ($($arg:tt)*) => (print!("{}\n", format_args!($($arg)*)))
}

#[macro_export]
macro_rules! eprintln {
    () => (print!("\n"));
    ($($arg:tt)*) => (eprint!("{}\n", format_args!($($arg)*)))
}

/// Overrides the default `eprint!` macro.
#[macro_export]
macro_rules! eprint {
    ($($arg:tt)*) => (#[cfg(debug_assertions)]
    $crate::_eprint_args(format_args!($($arg)*)));
}

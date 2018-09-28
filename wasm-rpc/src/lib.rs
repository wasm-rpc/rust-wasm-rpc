#![no_std]
#![feature(
    alloc,
    alloc_error_handler,
    core_intrinsics,
    panic_implementation,
    try_trait,
    )]
#![cfg_attr(debug_assertions, feature(set_stdio))]

extern crate alloc;
extern crate cbor_no_std;
#[cfg(test)]
extern crate ellipticoin_test_framework as ellipticoin;
mod error;
mod memory;
mod response;
mod pointer;

use memory::ptr_from_vec;
pub use error::{Error};
pub use response::{Responsable, Bytes};
pub use pointer::{
    Pointer,
    Referenceable,
    Dereferenceable,
};

#[cfg(debug_assertions)]
mod hook;
#[cfg(debug_assertions)]
pub use hook::{
    _print_args,
    _eprint_args,
};
#[cfg(debug_assertions)]
pub use hook::{hook};
#[cfg(debug_assertions)]
#[macro_use]
extern crate std;

// Note these must be defined in lib.rs
// https://users.rust-lang.org/t/how-to-export-rust-functions-from-child-module/11057/7
use alloc::vec::Vec;
pub unsafe fn dealloc(ptr: *mut u8, old_size: usize) {
    Vec::from_raw_parts(ptr, 0, old_size);
}
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

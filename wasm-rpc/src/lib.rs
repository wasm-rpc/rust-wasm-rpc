pub extern crate serde;
pub extern crate serde_cbor;
pub mod error;
pub mod pointer;
pub use pointer::Pointer;

// Note these must be defined in lib.rs
// https://users.rust-lang.org/t/how-to-export-rust-functions-from-child-module/11057/7
#[no_mangle]
pub unsafe fn __free(ptr: *mut u8) {
    pointer::free(ptr);
}

#[no_mangle]
pub unsafe fn __malloc(size: usize) -> *mut u8 {
    pointer::malloc(size)
}

#![no_std]
#![feature(
    try_trait,
    alloc,
    core_intrinsics,
    alloc_error_handler,
    panic_implementation,
    )]
#[macro_use] extern crate alloc;
extern crate cbor_no_std;
#[cfg(test)]
extern crate ellipticoin_test_framework as ellipticoin;
mod error;
pub mod memory;
pub use error::{Error};
use alloc::string::String;
use alloc::vec::Vec;
use cbor_no_std::{from_bytes, to_bytes, Value};
use core::mem;
use core::slice;
use core::intrinsics::transmute;

pub const LENGTH_BYTE_COUNT: usize = 4;

pub type Pointer = *const u8;

pub unsafe trait Dereferenceable {
    fn as_raw_bytes(&self) -> Vec<u8>;
    fn to_bytes(&self) -> Vec<u8>;
    fn to_int(&self) -> u64;
    fn to_string(&self) -> String;
    fn to_array(&self) -> Vec<Value>;
}

unsafe impl Dereferenceable for Pointer {
    fn as_raw_bytes(&self) -> Vec<u8> {
        let length_slice = unsafe { slice::from_raw_parts(self.offset(0) as *const u8, LENGTH_BYTE_COUNT as usize) };
        let mut length_slice_four: [u8; 4] = [0; 4];
        length_slice_four.copy_from_slice(&length_slice[..]);
        let length = unsafe{transmute::<[u8; 4], u32>(length_slice_four)};

        unsafe {
            slice::from_raw_parts(self.offset(LENGTH_BYTE_COUNT as isize) as *const u8, length as usize).to_vec()
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        from_bytes(self.as_raw_bytes()).as_bytes().unwrap().to_vec()
    }

    fn to_int(&self) -> u64 {
        from_bytes(self.as_raw_bytes()).as_int().unwrap()
    }

    fn to_string(&self) -> String {
        let name = from_bytes(self.as_raw_bytes());
        name.as_string().unwrap().clone()
    }

    fn to_array(&self) -> Vec<Value> {
        let name = from_bytes(self.as_raw_bytes());
        name.as_array().unwrap().clone()
    }
}

pub unsafe trait Referenceable {
    fn as_pointer(&self) -> Pointer;
}

unsafe impl Referenceable for Vec<u8> {
    fn as_pointer(&self) -> Pointer {
        let mut value = self.clone();
        let mut value_and_length = unsafe{transmute::<u32, [u8; 4]>(value.len() as u32)}.to_vec();
        value_and_length.append(&mut value);
        let value_and_length_ptr = value_and_length.as_ptr();
        mem::forget(value_and_length);
        value_and_length_ptr
    }
}

unsafe impl Referenceable for String {
    fn as_pointer(&self) -> Pointer {
        self.as_bytes().to_vec().as_pointer()
    }
}

pub trait Responsable {
    fn to_response(self) -> Pointer;
}


impl Responsable for Result<Value, Error> {
    fn to_response(self: Result<Value, Error>) -> Pointer {
        let error_code = match self {
            Err(error) => error.code,
            _ => 0,
        };

        let mut return_value = match self {
            Ok(Value::Null) => vec![],
            Ok(value) => to_bytes(value),
            Err(error) => to_bytes(Value::String(error.message.into())),
        };

        let mut error_code_bytes = unsafe{transmute::<u32, [u8; 4]>(error_code as u32)}.to_vec();

        error_code_bytes.append(&mut return_value);
        error_code_bytes.as_pointer()
    }
}

impl Responsable for Result<(), Error> {
    fn to_response(self) -> Pointer {
        match self {
            Ok(value) => (Ok::<Value, Error>(value.into())).to_response(),
            Err(error) => (Err::<Value, Error>(error)).to_response()
        }
    }
}

impl Responsable for Result<u32, Error> {
    fn to_response(self) -> Pointer {
        match self {
            Ok(value) => (Ok::<Value, Error>(value.into())).to_response(),
            Err(error) => (Err::<Value, Error>(error)).to_response()
        }
    }
}

impl Responsable for Result<u64, Error> {
    fn to_response(self) -> Pointer {
        match self {
            Ok(value) => (Ok::<Value, Error>(value.into())).to_response(),
            Err(error) => (Err::<Value, Error>(error)).to_response()
        }
    }
}

impl Responsable for Result<Vec<u8>, Error> {
    fn to_response(self) -> Pointer {
        match self {
            Ok(value) => (Ok::<Value, Error>(value.into())).to_response(),
            Err(error) => (Err::<Value, Error>(error)).to_response()
        }
    }
}

pub trait Bytes<T> {
    fn value(&self) -> T;
}

impl Bytes<u64> for Vec<u8> {
    fn value(&self) -> u64 {
        if self.len() == 8 {
            let mut slice: [u8; 8] = [0; 8];
            slice.copy_from_slice(&self[..]);
            unsafe{transmute::<[u8; 8], u64>(slice)}
        } else {
            0
        }
    }
}

impl Bytes<u32> for Vec<u8> {
    fn value(&self) -> u32 {
        if self.len() == 4 {
            let mut slice: [u8; 4] = [0; 4];
            slice.copy_from_slice(&self[..]);
            unsafe{transmute::<[u8; 4], u32>(slice)}
        } else {
            0
        }
    }
}

impl From<core::option::NoneError> for Error {
    fn from(_value: core::option::NoneError) -> Error {
        Error {
            code: 1,
            message: "None Error",
        }
    }
}

use core::intrinsics::transmute;
use error::Error;
use alloc::vec::Vec;
use cbor_no_std::{to_bytes, Value};
use pointer::{
    Pointer,
};
use pointer::Referenceable;

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
            Ok(Value::Null) => Vec::new(),
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

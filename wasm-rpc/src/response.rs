use abort::AbortResultExt;
use error::Error;
use pointer::Pointer;
use pointer::Referenceable;
use serde_cbor::{to_vec, Value};
use std::mem::transmute;

pub trait Responsable {
    fn to_response(self) -> Pointer;
}

impl<V: Into<Value>> Responsable for Result<V, Error> {
    fn to_response(self: Result<V, Error>) -> Pointer {
        to_vec(&self.unwrap_or_abort().into()).unwrap_or_abort().as_pointer()
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
            unsafe { transmute::<[u8; 8], u64>(slice) }
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
            unsafe { transmute::<[u8; 4], u32>(slice) }
        } else {
            0
        }
    }
}

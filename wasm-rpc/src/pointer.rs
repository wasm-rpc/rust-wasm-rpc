use serde_cbor::{from_slice, ObjectKey, Value};
use std::collections::BTreeMap;
use std::mem::transmute;
use std::{mem, slice};

pub const LENGTH_BYTE_COUNT: usize = 4;

pub type Pointer = *const u8;

pub unsafe trait Dereferenceable {
    fn as_raw_bytes(&self) -> Vec<u8>;
    fn to_bytes(&self) -> Vec<u8>;
    fn to_i64(&self) -> i64;
    fn to_string(&self) -> String;
    fn to_array(&self) -> Vec<Value>;
    fn to_object(&self) -> BTreeMap<ObjectKey, Value>;
}

unsafe impl Dereferenceable for Pointer {
    fn as_raw_bytes(&self) -> Vec<u8> {
        let length_slice = unsafe {
            slice::from_raw_parts(self.offset(0) as *const u8, LENGTH_BYTE_COUNT as usize)
        };
        let mut length_slice_four: [u8; 4] = [0; 4];
        length_slice_four.copy_from_slice(&length_slice[..]);
        let length = unsafe { transmute::<[u8; 4], u32>(length_slice_four) };

        unsafe {
            slice::from_raw_parts(
                self.offset(LENGTH_BYTE_COUNT as isize) as *const u8,
                length as usize,
            )
            .to_vec()
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let value: Value = from_slice(&self.as_raw_bytes()).unwrap();
        value.as_bytes().expect("expected bytes").to_vec()
    }

    fn to_i64(&self) -> i64 {
        let value: Value = from_slice(&self.as_raw_bytes()).unwrap();
        value.as_i64().expect("expected i64") as i64
    }

    fn to_string(&self) -> String {
        let value: Value = from_slice(&self.as_raw_bytes()).unwrap();
        value.as_string().unwrap().clone()
    }

    fn to_array(&self) -> Vec<Value> {
        let name: Value = from_slice(&self.as_raw_bytes()).unwrap();
        name.as_array().expect("expected array").clone()
    }

    fn to_object(&self) -> BTreeMap<ObjectKey, Value> {
        let object: Value = from_slice(&self.as_raw_bytes()).unwrap();
        object.as_object().expect("expected object").clone()
    }
}

pub unsafe trait Referenceable {
    fn as_pointer(&self) -> Pointer;
}

unsafe impl Referenceable for Vec<u8> {
    fn as_pointer(&self) -> Pointer {
        let mut value = self.clone();
        let mut value_and_length =
            unsafe { transmute::<u32, [u8; 4]>(value.len() as u32) }.to_vec();
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

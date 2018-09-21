use core::intrinsics::transmute;
use alloc::vec::Vec;
use alloc::string::String;
use core::{
    mem,
    slice,
};
use cbor_no_std::{from_bytes, Value};

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

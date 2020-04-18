use abort::AbortResultExt;
use memory::{ptr_to_u32, LENGTH_BYTE_COUNT};
use serde_cbor::Value;
use serde_cbor::to_vec;
use std::slice;

pub type Pointer = *const u8;

pub unsafe trait Dereferenceable {
    fn as_raw_bytes(&self) -> Vec<u8>;
}

unsafe impl Dereferenceable for Pointer {
    fn as_raw_bytes(&self) -> Vec<u8> {
        let length = ptr_to_u32(*self) as usize;
        unsafe { slice::from_raw_parts(self.offset(LENGTH_BYTE_COUNT), length).to_vec() }
    }
}

pub unsafe trait Referenceable {
    fn as_pointer(&self) -> Pointer;
}

unsafe impl Referenceable for Vec<u8> {
    fn as_pointer(&self) -> Pointer {
        [self.len().to_le_bytes().to_vec(), self.to_vec()]
            .concat()
            .as_ptr()
    }
}

unsafe impl Referenceable for String {
    fn as_pointer(&self) -> Pointer {
        self.as_bytes().to_vec().as_pointer()
    }
}

unsafe impl Referenceable for Value {
    fn as_pointer(&self) -> Pointer {
        to_vec(self).unwrap_or_abort().as_pointer()
    }
}

unsafe impl Referenceable for Vec<Value> {
    fn as_pointer(&self) -> Pointer {
        Value::Array(self.to_vec()).as_pointer()
    }
}

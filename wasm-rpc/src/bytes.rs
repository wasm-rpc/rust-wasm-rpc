use abort::AbortResultExt;
use std::mem::transmute;
use serde_cbor::{Value, from_slice, to_vec};

pub trait ToBytes {
    fn to_bytes(self: Self) -> Vec<u8>;
}

impl ToBytes for u8 {
    fn to_bytes(self: Self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
}

impl ToBytes for u64 {
    fn to_bytes(self: Self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
}

impl ToBytes for bool {
    fn to_bytes(self: Self) -> Vec<u8> {
        if self {
            vec![1]
        } else {
            vec![0]
        }
    }
}

impl ToBytes for &str {
    fn to_bytes(self: Self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }
}

impl ToBytes for String {
    fn to_bytes(self: Self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }
}

impl ToBytes for serde_bytes::ByteBuf {
    fn to_bytes(self: Self) -> Vec<u8> {
        self.to_vec()
    }
}

impl ToBytes for Vec<u8> {
    fn to_bytes(self: Self) -> Vec<u8> {
        self
    }
}

impl ToBytes for Vec<Value> {
    fn to_bytes(self: Self) -> Vec<u8> {
        to_vec(&self).unwrap_or_abort()
    }
}

impl ToBytes for Value {
    fn to_bytes(self: Self) -> Vec<u8> {
        to_vec(&self).unwrap_or_abort()
    }
}

pub trait FromBytes {
    fn from_bytes(_bytes: Vec<u8>) -> Self;
}

impl FromBytes for u64 {
    fn from_bytes(bytes: Vec<u8>) -> u64 {
        if bytes.len() == 8 {
            let mut slice: [u8; 8] = [0; 8];
            slice.copy_from_slice(&bytes[..]);
            unsafe { transmute::<[u8; 8], u64>(slice) }
        } else {
            0
        }
    }
}

impl FromBytes for Value {
    fn from_bytes(bytes: Vec<u8>) -> Self {
        if bytes.len() == 0 {
            Value::Null
        } else {
            from_slice(&bytes).unwrap_or_abort()
        }
    }
}

// impl FromBytes for Vec<Value> {
//     fn from_bytes(bytes: Vec<u8>) -> Vec<Value> {
//         if bytes.len() == 0 {
//             vec![]
//         } else {
//             let value: Value = from_slice(&bytes).unwrap_or_abort();
//             value.as_array().unwrap_or_abort().to_vec()
//         }
//     }
// }
//
impl FromBytes for Vec<u8> {
    fn from_bytes(bytes: Vec<u8>) -> Vec<u8> {
        bytes
    }
}

impl FromBytes for bool {
    fn from_bytes(bytes: Vec<u8>) -> bool {
        if bytes.len() == 1 {
            bytes[0] == 1
        } else if bytes.len() == 0 {
            false
        } else {
            panic!("Cannot convert {} bytes to bool", bytes.len())
        }
    }
}

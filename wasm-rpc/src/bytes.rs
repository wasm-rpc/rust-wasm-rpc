use std::mem::transmute;

pub trait ToBytes {
    fn to_bytes(self: Self) -> Vec<u8>;
}

impl ToBytes for u64 {
    fn to_bytes(self: Self) -> Vec<u8> {
        let slice = unsafe { transmute::<u64, [u8; 8]>(self) };
        slice.to_vec()
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

use error::Error;
use pointer::Pointer;
use pointer::Referenceable;
use serde_cbor::{to_vec, Value};
use std::mem::transmute;

pub trait Responsable {
    fn to_response(self) -> Pointer;
}

// impl<T: Into<Value>> Responsable for T {
//     fn to_response(self) -> Pointer {
//         Ok::<Value, Error>(self.into()).to_response()
//     }
// }

impl<V: Into<Value>> Responsable for Result<V, Error> {
    fn to_response(self: Result<V, Error>) -> Pointer {
        let (return_code, mut return_value) = match self {
            Ok(value) => (0, to_vec(&value.into()).unwrap()),
            Err(error) => {
                // let e: Errorable = *error;
                (
                    error.code,
                    to_vec(&Value::String(error.message.to_string())).unwrap(),
                )
            }
        };

        let mut return_code_bytes =
            unsafe { transmute::<u32, [u8; 4]>(return_code as u32) }.to_vec();

        return_code_bytes.append(&mut return_value);
        return_code_bytes.as_pointer()
    }
}

// impl Responsable for Result<(),  Error> {
//     fn to_response(self) -> Pointer {
//         match self {
//             Ok(value) => Ok(Value::Null).to_response(),
//             Err(error) => Err::<Value, Error>(error).to_response(),
//         }
//     }
// }

// impl Responsable for String {
//     fn to_response(self) -> Pointer {
//         Ok::<Value, Error>(self.into()).to_response()
//     }
// }

// impl Responsable for u64 {
//     fn to_response(self) -> Pointer {
//         Ok::<Value, Error>(self.into()).to_response()
//     }
// }

// impl Responsable for () {
//     fn to_response(self) -> Pointer {
//         Ok(Value::Null).to_response()
//     }
// }

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

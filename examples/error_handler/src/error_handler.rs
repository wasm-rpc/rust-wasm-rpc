use wasm_rpc::error::{Error, ErrorStruct};
use wasm_rpc::Value;
use wasm_rpc_macros::export;

pub const EXAMPLE_ERROR: ErrorStruct<'static> = Error {
    code: 1,
    message: "example error",
};

#[export]
pub fn success() -> String {
    "success".to_string()
}

#[export]
pub fn error() -> Result<Value, Error> {
    Err(EXAMPLE_ERROR)
}

#[export]
pub fn custom_panic() {
    panic!("panic number: {}", 42);
}

#[export]
pub fn panic() -> i64 {
    let v = vec![1, 2, 3];

    v[3]
}

#[export]
pub fn bad_arg(n: i64) -> i64 {
    n
}

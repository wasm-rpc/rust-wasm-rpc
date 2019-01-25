extern crate snap;
use wasm_rpc_macros::export;

use snap::Encoder;
use std::io;
use std::io::Write;

#[export]
pub fn run(
    data: Vec<u8>,
) -> Vec<u8> {
    Encoder::new().compress_vec(&data).unwrap()
}

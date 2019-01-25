#![feature(alloc, proc_macro_hygiene)]
extern crate wee_alloc;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
extern crate alloc;
extern crate serde_cbor;
extern crate wasm_rpc;
extern crate wasm_rpc_macros;

pub mod complex_arguments;

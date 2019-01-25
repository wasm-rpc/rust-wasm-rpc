use wasm_rpc::{BTreeMap, ObjectKey, Value};
use wasm_rpc_macros::export;

#[export]
pub fn run(int: u64, string: String, object: BTreeMap<ObjectKey, Value>, bytes: Vec<u8>) -> Value {
    let new_int = int + 1;
    let mut new_string = string.clone();
    new_string.push_str(" world");
    let mut new_object = object.clone();
    new_object.insert("key2".to_string().into(), "value2".to_string().into());
    let mut new_bytes = bytes.clone();
    new_bytes.extend_from_slice(&vec![4]);

    Value::Array(vec![
        new_int.into(),
        new_string.into(),
        new_object.into(),
        new_bytes.into(),
    ])
}

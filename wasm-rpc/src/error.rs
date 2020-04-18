use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Error {
    pub code: usize,
    pub message: &'static str,
}

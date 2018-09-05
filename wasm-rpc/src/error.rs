#[derive(Copy, Clone, Debug)]
pub struct Error {
    pub code: u32,
    pub message: &'static str,
}

pub type Error = ErrorStruct<'static>;
pub struct ErrorStruct<'a> {
    pub code: u32,
    pub message: &'a str,
}

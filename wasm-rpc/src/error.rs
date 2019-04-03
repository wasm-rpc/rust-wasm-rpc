pub type Error = ErrorStruct<'static>;
#[derive(Debug)]
pub struct ErrorStruct<'a> {
    pub code: u32,
    pub message: &'a str,
}

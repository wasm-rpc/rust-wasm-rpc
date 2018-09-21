use core::option::NoneError;

#[derive(Copy, Clone, Debug)]
pub struct Error {
    pub code: u32,
    pub message: &'static str,
}

impl From<NoneError> for Error {
    fn from(_value: NoneError) -> Error {
        Error {
            code: 1,
            message: "None Error",
        }
    }
}

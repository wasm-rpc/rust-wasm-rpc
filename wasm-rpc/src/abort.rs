use std::process;

pub trait AbortOptionExt<T> {
    fn unwrap_or_abort(self) -> T;
}

pub trait AbortResultExt<T, E> {
    fn unwrap_or_abort(self) -> T;
}

impl<T> AbortOptionExt<T> for Option<T> {
    fn unwrap_or_abort(self) -> T {
        match self {
            Some(x) => x,
            None => process::abort()
        }
    }
}

impl<T, E> AbortResultExt<T, E> for Result<T, E> {
    fn unwrap_or_abort(self) -> T {
        match self {
            Ok(x) => x,
            Err(_) => process::abort()
        }
    }
}

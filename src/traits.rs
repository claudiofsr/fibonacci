use std::fmt;

/// Result Extension
pub trait ResultExt<T> {
    /// If OK, unwrap Result<T, Error> to the value T.
    ///
    /// If Error, terminate the current process with error messages.
    fn unwrap_result(self) -> T;
}

impl<T, E: fmt::Display> ResultExt<T> for Result<T, E> {
    fn unwrap_result(self) -> T {
        match self {
            Ok(value) => value,
            Err(error) => {
                eprintln!("{error}");
                std::process::exit(1);
            }
        }
    }
}

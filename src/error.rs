use std::result;

use thiserror::Error;

/// A result of a function that may return a `Error`.
pub type Result<T> = result::Result<T, Error>;

/// Errors returned by the `libusb` library.
#[derive(Error, Debug, Copy, Clone, Eq, PartialEq)]
pub enum Error {
    #[error("libusbk error code: `{0}`")]
    Code(u32),
}

#[doc(hidden)]
pub(crate) fn from_libusbk(err: u32) -> Error {
    match err {
        i => Error::Code(i),
    }
}

#[doc(hidden)]
macro_rules! try_unsafe {
    ($x:expr) => {
        match unsafe { $x } {
            0 => {
                use winapi::um::errhandlingapi::GetLastError;

                let err: u32 = unsafe { GetLastError() };
                return Err($crate::error::from_libusbk(err));
            }
            _ => (),
        }
    };
}

pub(crate) use try_unsafe;

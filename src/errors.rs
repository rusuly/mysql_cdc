use std::{io, num::ParseIntError, str::Utf8Error, string::FromUtf8Error};

use hex::FromHexError;
use openssl::error::ErrorStack;

#[derive(Debug)]
pub enum Error {
    IoError(io::Error),
    Utf8Error(Utf8Error),
    FromUtf8Error(FromUtf8Error),
    FromHexError(FromHexError),
    ParseIntError(ParseIntError),
    ErrorStack(ErrorStack),
    String(String),
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::IoError(error)
    }
}

impl From<Utf8Error> for Error {
    fn from(error: Utf8Error) -> Self {
        Error::Utf8Error(error)
    }
}

impl From<FromUtf8Error> for Error {
    fn from(error: FromUtf8Error) -> Self {
        Error::FromUtf8Error(error)
    }
}

impl From<FromHexError> for Error {
    fn from(error: FromHexError) -> Self {
        Error::FromHexError(error)
    }
}

impl From<ParseIntError> for Error {
    fn from(error: ParseIntError) -> Self {
        Error::ParseIntError(error)
    }
}

impl From<ErrorStack> for Error {
    fn from(error: ErrorStack) -> Self {
        Error::ErrorStack(error)
    }
}

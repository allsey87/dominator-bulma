use core::fmt;
use std::{fmt::Display, error};

#[derive(Debug)]
pub enum Error {
    BadVariant
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::BadVariant => write!(f, "No such class for variant"),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

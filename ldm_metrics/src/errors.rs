use std::error;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    Generic(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Error::Generic(ref st) => write!(f, "{}", st),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Generic(ref st) => st,
        }
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        match *self {
            Error::Generic(_) => None,
        }
    }
}

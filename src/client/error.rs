use std::{fmt, io};
use crate::torrent;
use http::uri::InvalidUri;

#[derive(PartialEq, Debug)]
pub struct Error {
    details: String
}

impl Error {
    pub fn new(msg: String) -> Error {
        Self{details: msg}
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        &self.details
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::new(format!("{}", err))
    }
}

impl From<torrent::error::Error> for Error {
    fn from(err: torrent::error::Error) -> Self {
        Error::new(format!("{}", err))
    }
}

impl From<hyper::error::Error> for Error {
    fn from(err: hyper::error::Error) -> Self {
        Error::new(format!("{}", err))
    }
}

impl From<InvalidUri> for Error {
    fn from(err: InvalidUri) -> Self {
        Error::new(format!("{}", err))
    }
}

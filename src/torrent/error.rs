use std::fmt;
use crate::bencoding;

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

impl From<bencoding::error::Error> for Error {
    fn from(err: bencoding::error::Error) -> Self {
        Error::new(format!("{}", err))
    }
}

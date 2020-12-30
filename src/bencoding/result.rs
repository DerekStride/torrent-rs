use crate::bencoding::bencode::Bencode;
use crate::bencoding::error::Error;

pub type Result = std::result::Result<Bencode, Error>;

use std::collections::BTreeMap;
use std::{str, fmt};
use crate::bencoding::byte_string::ByteString;
use crate::bencoding::error::Error;

#[derive(PartialEq, Eq, Debug)]
pub enum Bencode {
    Empty,
    Number(i64),
    ByteString(Vec<u8>),
    List(ListVec),
    Dict(DictMap),
}

pub type ListVec = Vec<Bencode>;
pub type DictMap = BTreeMap<ByteString, Bencode>;

impl Bencode {
    pub fn remove(self, key: &str) -> Result<Bencode, Error> {
        let mut dict = match self {
            Bencode::Dict(d) => d,
            _ => return Err(Error::new("Bencode is not a dict".to_string())),
        };

        match dict.remove(&ByteString::from_str(key)) {
            Some(value) => Ok(value),
            None => return Err(Error::new(format!("\"{}\" key is not present in torrent file.", key))),
        }
    }

    pub fn remove_bytestring(self, key: &str) -> Result<Vec<u8>, Error> {
        let mut dict = match self {
            Bencode::Dict(d) => d,
            _ => return Err(Error::new("Bencode is not a dict".to_string())),
        };

        let index = &ByteString::from_str(key);

        if let Some(value) = dict.get(index) {
            if let Bencode::ByteString(_) = value {
                match dict.remove(index) {
                    Some(Bencode::ByteString(s)) => Ok(s),
                    Some(_) => Err(Error::new(format!("Something went wrong removing \"{}\" from Bencode struct, value was not a ByteString.", key))),
                    None => Err(Error::new(format!("Something went wrong removing \"{}\" from Bencode struct.", key))),
                }
            } else {
                Err(Error::new(format!("\"{}\" value is not a ByteString", key)))
            }
        } else {
            Err(Error::new(format!("\"{}\" key is not present in torrent file.", key)))
        }
    }

    pub fn get_string(&self, key: &str) -> Result<String, Error> {
        let dict = match self {
            Bencode::Dict(d) => d,
            _ => return Err(Error::new("Bencode is not a dict.".to_string())),
        };

        let bencode_value = match dict.get(&ByteString::from_str(key)) {
            Some(value) => value,
            None => return Err(Error::new(format!("\"{}\" key is not present in torrent file.", key))),
        };

        let byte_string = match bencode_value {
            Bencode::ByteString(s) => s,
            _ => return Err(Error::new(format!("\"{}\" value is not a ByteString.", key))),
        };

        let bytes: &[u8] = &byte_string;

        match str::from_utf8(bytes) {
            Ok(utf8) => Ok(utf8.to_string()),
            Err(_) => Err(Error::new(format!("\"{}\" not valid utf-8.", key))),
        }
    }

    pub fn get_number(&self, key: &str) -> Result<i64, Error> {
        let dict = match self {
            Bencode::Dict(d) => d,
            _ => return Err(Error::new("Bencode is not a dict.".to_string())),
        };

        let bencode_value = match dict.get(&ByteString::from_str(key)) {
            Some(value) => value,
            None => return Err(Error::new(format!("\"{}\" key is not present in torrent file.", key))),
        };

        match bencode_value {
            Bencode::Number(s) => Ok(*s),
            _ => Err(Error::new(format!("\"{}\" value is not an i64.", key))),
        }
    }
}

impl fmt::Display for Bencode {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        format(fmt, self)
    }
}

fn format(fmt: &mut fmt::Formatter, v: &Bencode) -> fmt::Result {
    match *v {
        Bencode::Empty => { Ok(()) }
        Bencode::Number(v) => write!(fmt, "{}", v),
        Bencode::ByteString(ref v) => fmt_bytestring(v, fmt),
        Bencode::List(ref v) => {
            write!(fmt, "[")?;
            let mut first = true;
            for value in v.iter() {
                if first {
                    first = false;
                } else {
                    write!(fmt, ", ")?;
                }
                write!(fmt, "{}", *value)?;
            }
            write!(fmt, "]")
        }
        Bencode::Dict(ref v) => {
            write!(fmt, "{{")?;
            let mut first = true;
            for (key, value) in v.iter() {
                if first {
                    first = false;
                } else {
                    write!(fmt, ", ")?;
                }
                write!(fmt, "{}: {}", *key, *value)?;
            }
            write!(fmt, "}}")
        }
    }
}

#[inline]
fn fmt_bytestring(s: &[u8], fmt: &mut fmt::Formatter) -> fmt::Result {
  match str::from_utf8(s) {
    Ok(utf8_str) => write!(fmt, "s\"{}\"", utf8_str),
    // Err(..) => write!(fmt, "s[{}]", s.len()),
    Err(..) => write!(fmt, "s{:?}", s),
  }
}

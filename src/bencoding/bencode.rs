use std::collections::BTreeMap;
use std::{str, fmt};
use crate::bencoding::byte_string::ByteString;

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
    // pub fn get(self, key: &str) -> Result<Bencode, String> {
    //     let dict = match self {
    //         Bencode::Dict(d) => d,
    //         _ => return Err("Bencode is not a dict".to_string()),
    //     };

    //     match dict.get(&ByteString::from_str(key)) {
    //         Some(value) => Ok(*value),
    //         None => return Err(format!("\"{}\" key is not present in torrent file.", key)),
    //     }
    // }

    pub fn get_bytestring(&self, key: &str) -> Result<Vec<u8>, String> {
        let dict = match self {
            Bencode::Dict(d) => d,
            _ => return Err("Bencode is not a dict".to_string()),
        };

        let bencode_value = match dict.get(&ByteString::from_str(key)) {
            Some(value) => value,
            None => return Err(format!("\"{}\" key is not present in torrent file.", key)),
        };

        match bencode_value {
            Bencode::ByteString(s) => Ok(*s), // <--- Error
            // cannot move out of `*s` which is behind a shared reference
            // move occurs because `*s` has type `std::vec::Vec<u8>`, which does not implement the `Copy` traitrustc(E0507)
            _ => return Err(format!("\"{}\" value is not a ByteString", key)),
        }
    }

    pub fn get_string(&self, key: &str) -> Result<String, String> {
        let dict = match self {
            Bencode::Dict(d) => d,
            _ => return Err("Bencode is not a dict.".to_string()),
        };

        let bencode_value = match dict.get(&ByteString::from_str(key)) {
            Some(value) => value,
            None => return Err(format!("\"{}\" key is not present in torrent file.", key)),
        };

        let byte_string = match bencode_value {
            Bencode::ByteString(s) => s,
            _ => return Err(format!("\"{}\" value is not a ByteString.", key)),
        };

        let bytes: &[u8] = &byte_string;

        match str::from_utf8(bytes) {
            Ok(utf8) => Ok(utf8.to_string()),
            Err(_) => Err(format!("\"{}\" not valid utf-8.", key)),
        }
    }

    pub fn get_number(&self, key: &str) -> Result<i64, String> {
        let dict = match self {
            Bencode::Dict(d) => d,
            _ => return Err("Bencode is not a dict.".to_string()),
        };

        let bencode_value = match dict.get(&ByteString::from_str(key)) {
            Some(value) => value,
            None => return Err(format!("\"{}\" key is not present in torrent file.", key)),
        };

        match bencode_value {
            Bencode::Number(s) => Ok(*s),
            _ => Err(format!("\"{}\" value is not an i64.", key)),
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
    Err(..) => write!(fmt, "s{:?}", s),
  }
}

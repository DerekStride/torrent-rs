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

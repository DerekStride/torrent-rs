use std::{str, fmt};

#[derive(Eq, PartialEq, Clone, Ord, PartialOrd, Hash, Debug)]
pub struct ByteString(Vec<u8>);

impl ByteString {
    pub fn from_vec(s: Vec<u8>) -> ByteString {
        ByteString(s)
    }

    pub fn as_slice(&self) -> &[u8] {
      match self {
        &ByteString(ref v)  => &v[..]
      }
    }

    pub fn unwrap(self) -> Vec<u8> {
        let ByteString(v) = self;
        v
    }
}

impl fmt::Display for ByteString {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &ByteString(ref v) => fmt_bytestring(&v[..], fmt),
        }
    }
}

#[inline]
fn fmt_bytestring(s: &[u8], fmt: &mut fmt::Formatter) -> fmt::Result {
  match str::from_utf8(s) {
    Ok(s) => write!(fmt, "s\"{}\"", s),
    Err(..) => write!(fmt, "s{:?}", s),
  }
}

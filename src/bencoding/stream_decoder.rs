use std::{str, io};
// use std::{str, io, cmp};
use std::iter::{Iterator, Peekable};
// use std::io::Read;

use crate::bencoding::bencode::{Bencode, ListVec, DictMap};
use crate::bencoding::error::Error;
use crate::bencoding::result::Result;
use crate::bencoding::byte_string;


type FileByte = std::result::Result<u8, io::Error>;
const BENCODE_EOF_ERROR: &str = "Unexpected end of stream.";

pub fn decode(data: &mut dyn Iterator<Item=FileByte>) -> Result {
    decode_internal(&mut data.peekable())
}

fn decode_internal(data: &mut Peekable<&mut dyn Iterator<Item=FileByte>>) -> Result {
    let numbers = [b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9'];
    match data.peek() {
        Some(Ok(b'i')) => decode_int(data),
        Some(Ok(b'l')) => decode_list(data),
        Some(Ok(b'd')) => decode_dictionary(data),
        Some(Ok(byte)) => {
            if numbers.contains(&byte) {
                decode_str(data)
            } else {
                Ok(Bencode::Empty)
            }
        },
        Some(Err(e)) => Err(Error::new(format!("{}", e))),
        None => Ok(Bencode::Empty),
    }
}

fn next(data: &mut dyn Iterator<Item=FileByte>) -> std::result::Result<u8, Error> {
    match data.next() {
        Some(Ok(v)) => Ok(v),
        Some(Err(e)) => Err(e.into()),
        None => Err(Error::new(BENCODE_EOF_ERROR.to_string())),
    }
}

fn decode_str(data: &mut Peekable<&mut dyn Iterator<Item=FileByte>>) -> Result {
    let mut length_as_bytes = Vec::new();
    let mut value = next(data)?;

    while value != b':' {
        length_as_bytes.push(value);
        value = next(data)?;
    }

    let length_str = str::from_utf8(&length_as_bytes)?;
    let length = length_str.parse::<usize>()?;

    if length == 0 {
        return Ok(Bencode::ByteString(Vec::<u8>::new()))
    };

    let mut counter = 0;
    let mut byte_string = Vec::new();

    while counter < length {
        value = next(data)?;
        byte_string.push(value);
        counter += 1;
    }

    return Ok(Bencode::ByteString(byte_string))
}

fn decode_int(data: &mut Peekable<&mut dyn Iterator<Item=FileByte>>) -> Result {
    next(data)?;
    let mut bencode_number_as_bytes = Vec::new();
    let mut value = next(data)?;

    while value != b'e' {
        bencode_number_as_bytes.push(value);
        value = next(data)?;
    }

    let bencode_number_as_str = str::from_utf8(&bencode_number_as_bytes)?;
    let number = bencode_number_as_str.parse::<i64>()?;

    return Ok(Bencode::Number(number))
}

fn peek(data: &mut Peekable<&mut dyn Iterator<Item= FileByte>>) ->  std::result::Result<u8, Error> {
    match data.peek() {
        Some(Ok(v)) => Ok(*v),
        Some(Err(e)) => Err(Error::new(format!("{}", e))),
        None => Err(Error::new("Unexpected end of stream.".to_string())),
    }
}

fn decode_list(data: &mut Peekable<&mut dyn Iterator<Item=FileByte>>) -> Result {
    next(data)?;
    let mut value = peek(data)?;
    let mut list = ListVec::new();

    while value != b'e' {
        let result = decode_internal(data)?;
        list.push(result);
        value = peek(data)?;
    }

    return Ok(Bencode::List(list))
}

fn decode_dictionary(data: &mut Peekable<&mut dyn Iterator<Item=FileByte>>) -> Result {
    next(data)?;
    let mut value = peek(data)?;
    let mut dict = DictMap::new();

    while value != b'e' {
        let result = decode_internal(data)?;
        let key = match result {
            Bencode::ByteString(v) => byte_string::ByteString::from_vec(v),
            _ => return Err(Error::new(format!("Dictionary key was `{}`, expected a ByteString", result))),
        };

        let bencode_value = decode_internal(data)?;
        dict.insert(key, bencode_value);
        value = peek(data)?;
    }

    return Ok(Bencode::Dict(dict))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Read;
    use crate::bencoding::bencode::{Bencode, ListVec, DictMap};
    use crate::bencoding::byte_string;

    fn decode(data: Vec<u8>) -> Result {
        super::decode(&mut data.bytes())
    }

    #[test]
    fn test_can_decode_an_empty_dictionary() {
        let s = b"de".to_vec();
        let result = decode(s);

        assert_eq!(Bencode::Dict(DictMap::new()), result.unwrap());
    }

    #[test]
    fn test_can_decode_a_malformed_dictionary() {
        let s = b"d".to_vec();
        let result = match decode(s) {
            Ok(_) => panic!("result should be an error"),
            Err(e) => e,
        };

        assert_eq!(Error::new(BENCODE_EOF_ERROR.to_string()), result);
    }

    #[test]
    fn test_can_decode_a_dictionary_with_invalid_key() {
        let s = b"d3:fooi-4ei3e4:spam".to_vec();
        let result = match decode(s) {
            Ok(_) => panic!("result should be an error"),
            Err(e) => e,
        };

        assert_eq!(Error::new("Dictionary key was `3`, expected a ByteString".to_string()), result);
    }

    #[test]
    fn test_can_decode_a_dictionary_with_one_item() {
        let s = b"d4:spami3ee".to_vec();
        let result = decode(s);

        let mut d = DictMap::new();
        d.insert(
            byte_string::ByteString::from_vec(b"spam".to_vec()),
            Bencode::Number(3)
        );

        assert_eq!(Bencode::Dict(d), result.unwrap());
    }

    #[test]
    fn test_can_decode_a_dictionary_with_two_items() {
        let s = b"d4:spami3e3:fool4:spami3eee".to_vec();
        let result = decode(s);

        let v = vec![
            Bencode::ByteString(b"spam".to_vec()),
            Bencode::Number(3),
        ];

        let mut d = DictMap::new();
        d.insert(
            byte_string::ByteString::from_vec(b"spam".to_vec()),
            Bencode::Number(3)
        );
        d.insert(
            byte_string::ByteString::from_vec(b"foo".to_vec()),
            Bencode::List(v)
        );

        assert_eq!(Bencode::Dict(d), result.unwrap());
    }

    #[test]
    fn test_can_decode_an_empty_list() {
        let s = b"le".to_vec();
        let result = decode(s);

        assert_eq!(Bencode::List(ListVec::new()), result.unwrap())
    }

    #[test]
    fn test_can_decode_a_malformed_list() {
        let s = b"l".to_vec();
        let result = match decode(s) {
            Ok(_) => panic!("result should be an error"),
            Err(e) => e,
        };

        assert_eq!(Error::new(BENCODE_EOF_ERROR.to_string()), result);
    }

    #[test]
    fn test_can_decode_a_list_with_one_item() {
        let s = b"li3ee".to_vec();
        let result = decode(s);

        let v = vec![
            Bencode::Number(3),
        ];

        assert_eq!(Bencode::List(v), result.unwrap())
    }

    #[test]
    fn test_can_decode_a_list_with_two_items() {
        let s = b"l4:spami3ee".to_vec();
        let result = decode(s);

        let v = vec![
            Bencode::ByteString(b"spam".to_vec()),
            Bencode::Number(3),
        ];

        assert_eq!(Bencode::List(v), result.unwrap())
    }

    #[test]
    fn test_can_decode_a_list_with_a_list() {
        let s = b"ll4:spami3eee".to_vec();
        let result = decode(s);

        let l = vec![
            Bencode::ByteString(b"spam".to_vec()),
            Bencode::Number(3),
        ];
        let v = vec![Bencode::List(l)];

        assert_eq!(Bencode::List(v), result.unwrap())
    }

    #[test]
    fn test_can_decode_a_positive_int() {
        let s = b"i13e".to_vec();
        let result = decode(s);

        assert_eq!(Bencode::Number(13), result.unwrap());
    }

    #[test]
    fn test_can_decode_a_negative_int() {
        let s = b"i-137e".to_vec();
        let result = decode(s);

        assert_eq!(Bencode::Number(-137), result.unwrap());
    }

    #[test]
    fn test_can_decode_a_string() {
        let s = b"4:spam".to_vec();
        let result = decode(s);

        assert_eq!(Bencode::ByteString(b"spam".to_vec()), result.unwrap());
    }

    #[test]
    fn test_can_decode_a_string_shorter_than_the_size() {
        let s = b"10:1234567".to_vec();
        let result = match decode(s) {
            Ok(_) => panic!("result should be an error"),
            Err(e) => e,
        };

        assert_eq!(Error::new(BENCODE_EOF_ERROR.to_string()), result);
    }

    #[test]
    fn test_can_decode_a_malformed_empty_string() {
        let s = b"4:".to_vec();
        let result = match decode(s) {
            Ok(_) => panic!("result should be an error"),
            Err(e) => e,
        };

        assert_eq!(Error::new(BENCODE_EOF_ERROR.to_string()), result);
    }

    #[test]
    fn test_can_decode_an_empty_string() {
        let s = b"0:".to_vec();
        let result = decode(s);

        assert_eq!(Bencode::ByteString(b"".to_vec()), result.unwrap());
    }

    #[test]
    fn test_empty_input() {
        let s = b"".to_vec();
        let result = decode(s);

        assert_eq!(Bencode::Empty, result.unwrap());
    }

    #[test]
    fn test_invalid_utf8_string() {
        let s = b"2:\xc3\x28".to_vec();
        let result = decode(s);
        let bytes = vec![195, 40];

        assert_eq!(Bencode::ByteString(bytes), result.unwrap());
    }

    #[test]
    fn test_key_after_bytestring() {
        let s = b"d3:foo3:bar4:infod6:pieces3:z\xc3\x287:private1:1ee".to_vec();
        let result = decode(s);
        let bytes = vec![122, 195, 40];

        let mut info = DictMap::new();
        info.insert(
            byte_string::ByteString::from_vec(b"pieces".to_vec()),
            Bencode::ByteString(bytes),
        );
        info.insert(
            byte_string::ByteString::from_vec(b"private".to_vec()),
            Bencode::ByteString(b"1".to_vec()),
        );

        let mut d = DictMap::new();
        d.insert(
            byte_string::ByteString::from_vec(b"foo".to_vec()),
            Bencode::ByteString(b"bar".to_vec()),
        );
        d.insert(
            byte_string::ByteString::from_vec(b"info".to_vec()),
            Bencode::Dict(info),
        );

        assert_eq!(Bencode::Dict(d), result.unwrap());
    }
}

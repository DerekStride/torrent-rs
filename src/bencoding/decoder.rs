use std::{str, cmp};

use crate::bencoding::bencode::{Bencode, ListVec, DictMap};
use crate::bencoding::byte_string;

pub fn decode(data: &[u8]) -> Bencode {
    decode_internal(data, 0).0
}

fn decode_internal(data: &[u8], index: usize) -> (Bencode, usize) {
    let numbers = [b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9'];
    let code = match data.get(index) {
        Some(&r) => r,
        None => return (Bencode::Empty, 0),
    };

    if numbers.contains(&code) {
        decode_str(data, index)
    } else if code == b'i' {
        decode_int(data, index)
    } else if code == b'l' {
        decode_list(data, index)
    } else if code == b'd' {
        decode_dictionary(data, index)
    } else {
        (Bencode::Empty, 0)
    }
}

fn decode_str(data: &[u8], index: usize) -> (Bencode, usize) {
    let slice = data.get(index..).unwrap();
    let i = match slice.iter().position(|&r| r == b':') {
        Some(val) => val,
        None => return (Bencode::Empty, 0),
    };

    let length_bytes = slice.get(..i).unwrap();
    let length_str = str::from_utf8(&length_bytes).unwrap();
    let length = match length_str.parse::<usize>() {
        Ok(val) => val,
        Err(_) => return (Bencode::Empty, 0),
    };
    let length = cmp::min(length, slice.len() - i - 1);

    if length == 0 {
        return (Bencode::ByteString(Vec::<u8>::new()), index+i+1)
    };

    let s = match slice.get((i+1)..(i+1+length)) {
        Some(val) => val,
        None => return (Bencode::Empty, 0),
    };

    (Bencode::ByteString(s.to_vec()), index+i+1+length)
}

fn decode_int(data: &[u8], index: usize) -> (Bencode, usize) {
    let slice = data.get(index..).unwrap();
    let i = match slice.iter().position(|&r| r == b'e') {
        Some(val) => val,
        None => return (Bencode::Empty, 0),
    };

    let number_bytes = slice.get(1..i).unwrap();
    let number_str = str::from_utf8(&number_bytes).unwrap();
    let number = match number_str.parse::<i64>() {
        Ok(val) => val,
        Err(_) => return (Bencode::Empty, 0),
    };

    (Bencode::Number(number), index + i + 1)
}

fn decode_list(data: &[u8], index: usize) -> (Bencode, usize) {
    let slice = data.get(index..).unwrap();
    let mut list = ListVec::new();
    let mut i = 1;
    let mut item : Bencode;

    loop {
        match slice.get(i) {
            Some(b'e') => break,
            Some(_) => {},
            None => return (Bencode::List(list), index + i),
        }

        let result = decode_internal(slice, i);

        item = result.0;
        i = result.1;
        list.push(item);
    }

    (Bencode::List(list), index + i)
}

fn decode_dictionary(data: &[u8], index: usize) -> (Bencode, usize) {
    let slice = data.get(index..).unwrap();
    let mut dict = DictMap::new();
    let mut i = 1;
    let mut key : byte_string::ByteString;
    let mut value : Bencode;

    loop {
        match slice.get(i) {
            Some(b'e') => break,
            Some(_) => {},
            None => return (Bencode::Dict(dict), index + i),
        }

        let result = decode_internal(slice, i);

        i = result.1;

        key = match result.0 {
            Bencode::ByteString(v) => byte_string::ByteString::from_vec(v),
            _ => return (Bencode::Dict(dict), index + i),
        };

        let result = decode_internal(slice, i); 

        value = result.0;
        i = result.1;

        dict.insert(key, value);
    }

    (Bencode::Dict(dict), index + i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_decode_an_empty_dictionary() {
        let s = b"de";
        let result = decode(s);

        assert_eq!(Bencode::Dict(DictMap::new()), result);
    }

    #[test]
    fn test_can_decode_a_malformed_dictionary() {
        let s = b"d";
        let result = decode(s);

        assert_eq!(Bencode::Dict(DictMap::new()), result);
    }

    #[test]
    fn test_can_decode_a_dictionary_with_invalid_key() {
        let s = b"d3:fooi-4ei3e4:spam";
        let result = decode(s);

        let mut d = DictMap::new();
        d.insert(
            byte_string::ByteString::from_vec(b"foo".to_vec()),
            Bencode::Number(-4)
        );

        assert_eq!(Bencode::Dict(d), result);
    }

    #[test]
    fn test_can_decode_a_dictionary_with_one_item() {
        let s = b"d4:spami3ee";
        let result = decode(s);

        let mut d = DictMap::new();
        d.insert(
            byte_string::ByteString::from_vec(b"spam".to_vec()),
            Bencode::Number(3)
        );

        assert_eq!(Bencode::Dict(d), result);
    }

    #[test]
    fn test_can_decode_a_dictionary_with_two_items() {
        let s = b"d4:spami3e3:fool4:spami3eee";
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

        assert_eq!(Bencode::Dict(d), result);
    }

    #[test]
    fn test_can_decode_an_empty_list() {
        let s = b"le";
        let result = decode(s);

        assert_eq!(Bencode::List(ListVec::new()), result)
    }

    #[test]
    fn test_can_decode_a_malformed_list() {
        let s = b"l";
        let result = decode(s);

        assert_eq!(Bencode::List(ListVec::new()), result)
    }

    #[test]
    fn test_can_decode_a_list_with_one_item() {
        let s = b"li3ee";
        let result = decode(s);

        let v = vec![
            Bencode::Number(3),
        ];

        assert_eq!(Bencode::List(v), result)
    }

    #[test]
    fn test_can_decode_a_list_with_two_items() {
        let s = b"l4:spami3ee";
        let result = decode(s);

        let v = vec![
            Bencode::ByteString(b"spam".to_vec()),
            Bencode::Number(3),
        ];

        assert_eq!(Bencode::List(v), result)
    }

    #[test]
    fn test_can_decode_a_list_with_a_list() {
        let s = b"ll4:spami3eee";
        let result = decode(s);

        let l = vec![
            Bencode::ByteString(b"spam".to_vec()),
            Bencode::Number(3),
        ];
        let v = vec![Bencode::List(l)];

        assert_eq!(Bencode::List(v), result)
    }

    #[test]
    fn test_can_decode_a_positive_int() {
        let s = b"i13e";
        let result = decode(s);

        assert_eq!(Bencode::Number(13), result);
    }

    #[test]
    fn test_can_decode_a_negative_int() {
        let s = b"i-137e";
        let result = decode(s);

        assert_eq!(Bencode::Number(-137), result);
    }

    #[test]
    fn test_can_decode_a_string() {
        let s = b"4:spam";
        let result = decode(s);

        assert_eq!(Bencode::ByteString(b"spam".to_vec()), result);
    }

    #[test]
    fn test_can_decode_a_string_shorter_than_the_size() {
        let s = b"10:1234567";
        let result = decode(s);

        assert_eq!(Bencode::ByteString(b"1234567".to_vec()), result);
    }

    #[test]
    fn test_can_decode_a_malformed_empty_string() {
        let s = b"4:";
        let result = decode(s);

        assert_eq!(Bencode::ByteString(b"".to_vec()), result);
    }

    #[test]
    fn test_can_decode_an_empty_string() {
        let s = b"0:";
        let result = decode(s);

        assert_eq!(Bencode::ByteString(b"".to_vec()), result);
    }

    #[test]
    fn test_empty_input() {
        let s = b"";
        let result = decode(s);

        assert_eq!(Bencode::Empty, result);
    }

    #[test]
    fn test_invalid_utf8_string() {
        let s = b"2:\xc3\x28";
        let result = decode(s);
        let bytes = vec![195, 40];

        assert_eq!(Bencode::ByteString(bytes), result);
    }
}

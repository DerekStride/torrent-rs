use crate::bencoding::bencode::{Bencode, ListVec, DictMap};

#[cfg(test)]
use crate::bencoding::byte_string::ByteString;

pub fn encode(data: Bencode) -> Vec<u8> {
    let mut buffer = Vec::<u8>::new();
    encode_internal(&mut buffer, data);
    return buffer;
}

fn encode_internal(buffer: &mut Vec<u8>, data: Bencode) {
    match data {
        Bencode::ByteString(s) => encode_str(buffer, s),
        Bencode::Number(n) => encode_int(buffer, n),
        Bencode::List(l) => encode_list(buffer, l),
        Bencode::Dict(d) => encode_dictionary(buffer, d),
        _ => {},
    }
}

fn encode_str(buffer: &mut Vec<u8>, data: Vec<u8>) {
    for &byte in data.len().to_string().as_bytes() {
        buffer.push(byte);
    }

    buffer.push(b':');

    for byte in data {
        buffer.push(byte);
    }
}

fn encode_int(buffer: &mut Vec<u8>, data: i64) {
    buffer.push(b'i');

    for &byte in data.to_string().as_bytes() {
        buffer.push(byte);
    }

    buffer.push(b'e');
}

fn encode_list(buffer: &mut Vec<u8>, data: ListVec) {
    buffer.push(b'l');

    for value in data {
        encode_internal(buffer, value);
    }

    buffer.push(b'e');
}

fn encode_dictionary(buffer: &mut Vec<u8>, data: DictMap) {
    buffer.push(b'd');

    for (key, value) in data {
        encode_internal(buffer, Bencode::ByteString(key.unwrap()));
        encode_internal(buffer, value);
    }

    buffer.push(b'e');
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_encode_an_empty_dictionary() {
        let dict = Bencode::Dict(DictMap::new());
        let result = encode(dict);

        assert_eq!(b"de".to_vec(), result);
    }

    #[test]
    fn test_can_encode_a_dictionary_with_one_item() {
        let mut d = DictMap::new();
        d.insert(
            ByteString::from_vec(b"spam".to_vec()),
            Bencode::Number(3)
        );
        let result = encode(Bencode::Dict(d));

        assert_eq!(b"d4:spami3ee".to_vec(), result);
    }

    #[test]
    fn test_can_encode_a_dictionary_with_two_items() {
        let mut d = DictMap::new();
        let v = vec![
            Bencode::ByteString(b"eggs".to_vec()),
            Bencode::Number(-4),
        ];
        d.insert(
            ByteString::from_vec(b"spam".to_vec()),
            Bencode::Number(3)
        );
        d.insert(
            ByteString::from_vec(b"foo".to_vec()),
            Bencode::List(v)
        );
        let result = encode(Bencode::Dict(d));

        assert_eq!(b"d3:fool4:eggsi-4ee4:spami3ee".to_vec(), result);
    }

    #[test]
    fn test_can_encode_an_empty_list() {
        let list = Bencode::List(ListVec::new());
        let result = encode(list);

        assert_eq!(b"le".to_vec(), result);
    }

    #[test]
    fn test_can_encode_a_list_with_one_item() {
        let v = vec![
            Bencode::Number(3),
        ];
        let list = Bencode::List(v);
        let result = encode(list);

        assert_eq!(b"li3ee".to_vec(), result);
    }

    #[test]
    fn test_can_encode_a_list_with_two_items() {
        let v = vec![
            Bencode::ByteString(b"spam".to_vec()),
            Bencode::Number(3),
        ];
        let list = Bencode::List(v);
        let result = encode(list);

        assert_eq!(b"l4:spami3ee".to_vec(), result);
    }

    #[test]
    fn test_can_encode_a_list_with_a_list() {
        let l = vec![
            Bencode::ByteString(b"spam".to_vec()),
            Bencode::Number(3),
        ];
        let v = vec![Bencode::List(l)];
        let list = Bencode::List(v);
        let result = encode(list);

        assert_eq!(b"ll4:spami3eee".to_vec(), result);
    }

    #[test]
    fn test_can_encode_a_positive_int() {
        let number = Bencode::Number(13);
        let result = encode(number);

        assert_eq!(b"i13e".to_vec(), result);
    }

    #[test]
    fn test_can_encode_a_negative_int() {
        let number = Bencode::Number(-137);
        let result = encode(number);

        assert_eq!(b"i-137e".to_vec(), result);
    }

    #[test]
    fn test_can_encode_a_string() {
        let byte_string = Bencode::ByteString(b"spam".to_vec());
        let result = encode(byte_string);

        assert_eq!(b"4:spam".to_vec(), result);
    }

    #[test]
    fn test_can_encode_an_empty_string() {
        let byte_string = Bencode::ByteString(b"".to_vec());
        let result = encode(byte_string);

        assert_eq!(b"0:".to_vec(), result);
    }

    #[test]
    fn test_invalid_utf8_string() {
        let byte_string = Bencode::ByteString(b"\xc3\x28".to_vec());
        let result = encode(byte_string);
        
        assert_eq!(b"2:\xc3\x28".to_vec(), result);
    }
}

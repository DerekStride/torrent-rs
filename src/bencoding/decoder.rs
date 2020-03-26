use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::{str, cmp};

#[derive(Debug, PartialEq, Eq)]
pub struct Result {
    string: Option<String>,
    integer: Option<isize>,
    list: Option<Vec<Result>>,
    dictionary: Option<HashMap<Result, Result>>,
}

impl Hash for Result {
    fn hash<H: Hasher>(&self, state: &mut H) {
        
        self.string.hash(state);
        self.integer.hash(state);
        self.list.hash(state);

        let dict = match &self.dictionary {
            Some(d)=> d,
            None => return,
        };

        for (key, value) in dict {
            key.hash(state);
            value.hash(state);
        }
    }
}

impl Result {
    pub fn empty() -> Self {
        Self {
            string: None,
            integer: None,
            list: None,
            dictionary: None,
        }
    }

    pub fn string(data: String) -> Self {
        Self {
            string: Some(data),
            integer: None,
            list: None,
            dictionary: None,
        }
    }

    pub fn integer(data: isize) -> Self {
        Self {
            string: None,
            integer: Some(data),
            list: None,
            dictionary: None,
        }
    }

    pub fn list(data: Vec<Result>) -> Self {
        Self {
            string: None,
            integer: None,
            list: Some(data),
            dictionary: None,
        }
    }

    pub fn dictionary(data: HashMap<Result, Result>) -> Self {
        Self {
            string: None,
            integer: None,
            list: None,
            dictionary: Some(data),
        }
    }

    #[cfg(test)]
    pub fn is_string(&self) -> bool {
        self.string != None
    }

    #[cfg(test)]
    pub fn is_integer(&self) -> bool {
        self.integer != None
    }

    #[cfg(test)]
    pub fn is_list(&self) -> bool {
        self.list != None
    }

    #[cfg(test)]
    pub fn is_dictionary(&self) -> bool {
        self.dictionary != None
    }

    #[cfg(test)]
    pub fn is_empty(&self) -> bool {
        self.string == None &&
            self.integer == None &&
            self.list == None &&
            self.dictionary == None
    }
}

pub fn decode(data: &[u8]) -> Result {
    decode_internal(data, 0).0
}

fn decode_internal(data: &[u8], index: usize) -> (Result, usize) {
    let numbers = [b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9'];
    let code = match data.get(index) {
        Some(&r) => r,
        None => return (Result::empty(), 0),
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
        (Result::empty(), 0)
    }
}

fn decode_str(data: &[u8], index: usize) -> (Result, usize) {
    let slice = data.get(index..).unwrap();
    let i = match slice.iter().position(|&r| r == b':') {
        Some(val) => val,
        None => return (Result::empty(), 0),
    };

    let length_bytes = slice.get(..i).unwrap();
    let length_str = str::from_utf8(&length_bytes).unwrap();
    let length = match length_str.parse::<usize>() {
        Ok(val) => val,
        Err(_) => return (Result::empty(), 0),
    };
    let length = cmp::min(length, slice.len() - i - 1);

    if length == 0 {
        return (Result::string("".to_string()), index+i+1)
    };

    let s = match slice.get((i+1)..(i+1+length)) {
        Some(val) => val,
        None => return (Result::empty(), 0),
    };

    let utf8_str = match str::from_utf8(s) {
        Ok(val) => val.to_string(),
        Err(_) => {
            return (Result::string("".to_string()), index+i+1+length);
        },
    };

    (Result::string(utf8_str), index+i+1+length)
}

fn decode_int(data: &[u8], index: usize) -> (Result, usize) {
    let slice = data.get(index..).unwrap();
    let i = match slice.iter().position(|&r| r == b'e') {
        Some(val) => val,
        None => return (Result::empty(), 0),
    };

    let number_bytes = slice.get(1..i).unwrap();
    let number_str = str::from_utf8(&number_bytes).unwrap();
    let number = match number_str.parse::<isize>() {
        Ok(val) => val,
        Err(_) => return (Result::empty(), 0),
    };

    (Result::integer(number), index + i + 1)
}

fn decode_list(data: &[u8], index: usize) -> (Result, usize) {
    let slice = data.get(index..).unwrap();
    let mut list = Vec::<Result>::new();
    let mut i = 1;
    let mut item : Result;

    loop {
        match slice.get(i) {
            Some(b'e') => break,
            Some(_) => {},
            None => return (Result::list(list), index + i),
        }

        let result = decode_internal(slice, i);

        item = result.0;
        i = result.1;
        list.push(item);
    }

    (Result::list(list), index + i)
}

fn decode_dictionary(data: &[u8], index: usize) -> (Result, usize) {
    let slice = data.get(index..).unwrap();
    let mut dict = HashMap::<Result, Result>::new();
    let mut i = 1;
    let mut key : Result;
    let mut value : Result;

    loop {
        match slice.get(i) {
            Some(b'e') => break,
            Some(_) => {},
            None => return (Result::dictionary(dict), index + i),
        }

        let result = decode_internal(slice, i);

        key = result.0;
        i = result.1;

        let result = decode_internal(slice, i); 

        value = result.0;
        i = result.1;

        dict.insert(key, value);
    }

    (Result::dictionary(dict), index + i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_decode_an_empty_dictionary() {
        let s = b"de";
        let result = decode(s);
        let d = HashMap::<Result, Result>::new();

        assert!(result.is_dictionary());
        assert_eq!(Some(d), result.dictionary);
    }

    #[test]
    fn test_can_decode_a_malformed_dictionary() {
        let s = b"d";
        let result = decode(s);

        assert!(result.is_dictionary());
        assert_eq!(Some(HashMap::<Result, Result>::new()), result.dictionary);
    }

    #[test]
    fn test_can_decode_a_dictionary_with_one_item() {
        let s = b"d4:spami3ee";
        let result = decode(s);

        let mut d = HashMap::<Result, Result>::new();
        d.insert(Result::string("spam".to_string()), Result::integer(3));

        assert!(result.is_dictionary());
        assert_eq!(Some(d), result.dictionary);
    }

    #[test]
    fn test_can_decode_a_dictionary_with_two_items() {
        let s = b"d4:spami3e3:fool4:spami3eee";
        let result = decode(s);

        let v = vec![
            Result::string("spam".to_string()),
            Result::integer(3),
        ];

        let mut d = HashMap::<Result, Result>::new();
        d.insert(Result::string("spam".to_string()), Result::integer(3));
        d.insert(Result::string("foo".to_string()), Result::list(v));

        assert!(result.is_dictionary());
        assert_eq!(Some(d), result.dictionary);
    }

    #[test]
    fn test_can_decode_an_empty_list() {
        let s = b"le";
        let result = decode(s);

        assert!(result.is_list());
        assert_eq!(Some(Vec::<Result>::new()), result.list);
    }

    #[test]
    fn test_can_decode_a_malformed_list() {
        let s = b"l";
        let result = decode(s);

        assert!(result.is_list());
        assert_eq!(Some(Vec::<Result>::new()), result.list);
    }

    #[test]
    fn test_can_decode_a_list_with_one_item() {
        let s = b"li3ee";
        let result = decode(s);

        let v = vec![
            Result::integer(3),
        ];

        assert!(result.is_list());
        assert_eq!(Some(v), result.list);
    }

    #[test]
    fn test_can_decode_a_list_with_two_items() {
        let s = b"l4:spami3ee";
        let result = decode(s);

        let v = vec![
            Result::string("spam".to_string()),
            Result::integer(3),
        ];

        assert!(result.is_list());
        assert_eq!(Some(v), result.list);
    }

    #[test]
    fn test_can_decode_a_list_with_a_list() {
        let s = b"ll4:spami3eee";
        let result = decode(s);

        let l = vec![
            Result::string("spam".to_string()),
            Result::integer(3),
        ];
        let v = vec![Result::list(l)];

        assert!(result.is_list());
        assert_eq!(Some(v), result.list);
    }

    #[test]
    fn test_can_decode_a_positive_int() {
        let s = b"i13e";
        let result = decode(s);

        assert!(result.is_integer());
        assert_eq!(Some(13), result.integer);
    }

    #[test]
    fn test_can_decode_a_negative_int() {
        let s = b"i-137e";
        let result = decode(s);

        assert!(result.is_integer());
        assert_eq!(Some(-137), result.integer);
    }

    #[test]
    fn test_can_decode_a_string() {
        let s = b"4:spam";
        let result = decode(s);

        assert!(result.is_string());
        assert_eq!(Some("spam".to_string()), result.string);
    }

    #[test]
    fn test_can_decode_a_string_shorter_than_the_size() {
        let s = b"10:1234567";
        let result = decode(s);

        assert!(result.is_string());
        assert_eq!(Some("1234567".to_string()), result.string);
    }

    #[test]
    fn test_can_decode_a_malformed_empty_string() {
        let s = b"4:";
        let result = decode(s);

        assert!(result.is_string());
        assert_eq!(Some("".to_string()), result.string);
    }

    #[test]
    fn test_can_decode_an_empty_string() {
        let s = b"0:";
        let result = decode(s);

        assert!(result.is_string());
        assert_eq!(Some("".to_string()), result.string);
    }

    #[test]
    fn test_empty_input() {
        let s = b"";
        let result = decode(s);

        assert!(result.is_empty());
        assert_eq!(Result::empty(), result);
    }
}

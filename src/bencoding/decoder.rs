// use std::collections::HashMap;
use regex::Regex;
use std::cmp;

#[derive(Debug, PartialEq)]
pub struct Result {
    string: Option<String>,
    integer: Option<isize>,
    list: Option<Vec<Result>>,
}

impl Result {
    pub fn empty() -> Self {
        Self {
            string: None,
            integer: None,
            list: None,
        }
    }

    pub fn string(data: String) -> Self {
        Self {
            string: Some(data),
            integer: None,
            list: None,
        }
    }

    pub fn integer(data: isize) -> Self {
        Self {
            string: None,
            integer: Some(data),
            list: None,
        }
    }

    pub fn list(data: Vec<Result>) -> Self {
        Self {
            string: None,
            integer: None,
            list: Some(data),
        }
    }

    pub fn is_string(&self) -> bool {
        self.string != None
    }

    pub fn is_integer(&self) -> bool {
        self.integer != None
    }

    pub fn is_list(&self) -> bool {
        self.list != None
    }

    pub fn is_empty(&self) -> bool {
        self.string == None &&
            self.integer == None
    }
}

pub fn decode(data: String) -> Result {
    decode_internal(data, 0).0
}

fn decode_internal(data: String, index: usize) -> (Result, usize) {
    let re = Regex::new(r"^\d+$").unwrap();
    let code = match data.get(index..(index+1)) {
        Some(r) => r,
        None => return (Result::empty(), 0),
    };

    if re.is_match(code) {
        decode_str(data, index)
    } else if code == "i" {
        decode_int(data, index)
    } else if code == "l" {
        decode_list(data, index)
    } else {
        (Result::empty(), 0)
    }
}

fn decode_str(data: String, index: usize) -> (Result, usize) {
    let slice = data.get(index..).unwrap();
    let i = match slice.find(':') {
        Some(val) => val,
        None => return (Result::empty(), 0),
    };

    let length = match slice.get(..i).unwrap().parse::<usize>() {
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

    (Result::string(s.to_string()), index+i+1+length)
}

fn decode_int(data: String, index: usize) -> (Result, usize) {
    let slice = data.get(index..).unwrap();
    let i = match slice.find('e') {
        Some(val) => val,
        None => return (Result::empty(), 0),
    };

    let number = match slice.get(1..i).unwrap().parse::<isize>() {
        Ok(val) => val,
        Err(_) => return (Result::empty(), 0),
    };

    (Result::integer(number), index + i + 1)
}

fn decode_list(data: String, index: usize) -> (Result, usize) {
    let slice = data.get(index..).unwrap();
    let mut list = Vec::<Result>::new();
    let mut i = 1;
    let mut item : Result;

    loop {
        match slice.get((i)..(i+1)) {
            Some("e") => break,
            Some(_) => {},
            None => return (Result::list(list), index + i),
        }

        let result = decode_internal(slice.to_string(), i);

        item = result.0;
        i = result.1;
        list.push(item);
    }

    (Result::list(list), index + i)
}

#[cfg(test)]
mod tests {
    use super::*;


    // # pp Becoding::Decoder.decode("4:spam")
    // # pp Becoding::Decoder.decode("i-3e")
    // # pp Becoding::Decoder.decode("i3e")
    // # pp Becoding::Decoder.decode("i0e")
    // # pp Becoding::Decoder.decode("l4:spam4:eggse")
    // # pp Becoding::Decoder.decode("li-4ei4e4:eggse")
    // # pp Becoding::Decoder.decode("d4:spaml1:a1:bee")
    // # pp Becoding::Decoder.decode("d3:cow3:moo4:spam4:eggse")

    #[test]
    fn test_can_decode_an_empty_list() {
        let s = "le".to_string();
        let result = decode(s);

        assert!(result.is_list());
        assert_eq!(Some(Vec::<Result>::new()), result.list);
    }

    #[test]
    fn test_can_decode_a_malformed_list() {
        let s = "l".to_string();
        let result = decode(s);

        assert!(result.is_list());
        assert_eq!(Some(Vec::<Result>::new()), result.list);
    }

    #[test]
    fn test_can_decode_a_list_with_one_item() {
        let s = "li3ee".to_string();
        let result = decode(s);

        let v = vec![
            Result::integer(3),
        ];

        assert!(result.is_list());
        assert_eq!(Some(v), result.list);
    }

    #[test]
    fn test_can_decode_a_list_with_two_items() {
        let s = "l4:spami3ee".to_string();
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
        let s = "ll4:spami3eee".to_string();
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
        let s = "i13e".to_string();
        let result = decode(s);

        assert!(result.is_integer());
        assert_eq!(Some(13), result.integer);
    }

    #[test]
    fn test_can_decode_a_negative_int() {
        let s = "i-137e".to_string();
        let result = decode(s);

        assert!(result.is_integer());
        assert_eq!(Some(-137), result.integer);
    }

    #[test]
    fn test_can_decode_a_string() {
        let s = "4:spam".to_string();
        let result = decode(s);

        assert!(result.is_string());
        assert_eq!(Some("spam".to_string()), result.string);
    }

    #[test]
    fn test_can_decode_a_string_shorter_than_the_size() {
        let s = "10:1234567".to_string();
        let result = decode(s);

        assert!(result.is_string());
        assert_eq!(Some("1234567".to_string()), result.string);
    }

    #[test]
    fn test_can_decode_a_malformed_empty_string() {
        let s = "4:".to_string();
        let result = decode(s);

        assert!(result.is_string());
        assert_eq!(Some("".to_string()), result.string);
    }

    #[test]
    fn test_can_decode_an_empty_string() {
        let s = "0:".to_string();
        let result = decode(s);

        assert!(result.is_string());
        assert_eq!(Some("".to_string()), result.string);
    }

    #[test]
    fn test_empty_input() {
        let s = "".to_string();
        let result = decode(s);

        assert!(result.is_empty());
        assert_eq!(Result::empty(), result);
    }
}

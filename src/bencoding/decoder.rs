// use std::collections::HashMap;
use regex::Regex;
use std::cmp;

#[derive(Debug, PartialEq)]
pub struct Result {
    string: Option<String>,
    integer: Option<isize>,
}

impl Result {
    pub fn empty() -> Self {
        Self {
            string: None,
            integer: None,
        }
    }

    pub fn string(data: String) -> Self {
        Self {
            string: Some(data),
            integer: None,
        }
    }

    pub fn integer(data: isize) -> Self {
        Self {
            string: None,
            integer: Some(data),
        }
    }

    pub fn is_string(&self) -> bool {
        self.string != None
    }

    pub fn is_integer(&self) -> bool {
        self.integer != None
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

#[cfg(test)]
mod tests {
    use super::*;

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

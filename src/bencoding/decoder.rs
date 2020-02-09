use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use regex::Regex;
use std::cmp;

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
    } else if code == "d" {
        decode_dictionary(data, index)
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

fn decode_dictionary(data: String, index: usize) -> (Result, usize) {
    let slice = data.get(index..).unwrap();
    let mut dict = HashMap::<Result, Result>::new();
    let mut i = 1;
    let mut key : Result;
    let mut value : Result;

    loop {
        match slice.get((i)..(i+1)) {
            Some("e") => break,
            Some(_) => {},
            None => return (Result::dictionary(dict), index + i),
        }

        let result = decode_internal(slice.to_string(), i);

        key = result.0;
        i = result.1;

        let result = decode_internal(slice.to_string(), i); 

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
        let s = "de".to_string();
        let result = decode(s);
        let d = HashMap::<Result, Result>::new();

        assert!(result.is_dictionary());
        assert_eq!(Some(d), result.dictionary);
    }

    #[test]
    fn test_can_decode_a_malformed_dictionary() {
        let s = "d".to_string();
        let result = decode(s);

        assert!(result.is_dictionary());
        assert_eq!(Some(HashMap::<Result, Result>::new()), result.dictionary);
    }

    #[test]
    fn test_can_decode_a_dictionary_with_one_item() {
        let s = "d4:spami3ee".to_string();
        let result = decode(s);

        let mut d = HashMap::<Result, Result>::new();
        d.insert(Result::string("spam".to_string()), Result::integer(3));

        assert!(result.is_dictionary());
        assert_eq!(Some(d), result.dictionary);
    }

    #[test]
    fn test_can_decode_a_dictionary_with_two_items() {
        let s = "d4:spami3e3:fool4:spami3eee".to_string();
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

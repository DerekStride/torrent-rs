use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::fmt;

#[derive(PartialEq, Eq)]
pub struct Result {
    pub string: Option<String>,
    pub integer: Option<isize>,
    pub list: Option<Vec<Result>>,
    pub dictionary: Option<HashMap<Result, Result>>,
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

    pub fn is_string(&self) -> bool {
        self.string != None
    }

    pub fn is_integer(&self) -> bool {
        self.integer != None
    }

    pub fn is_list(&self) -> bool {
        self.list != None
    }

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

impl fmt::Debug for Result {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_string() {
            return match &self.string {
                Some(s) => f.write_fmt(format_args!("\"{}\"", s)),
                None => panic!("is_string() but has no string!"),
            };
        } else if self.is_integer() {
            return match &self.integer {
                Some(s) => f.write_fmt(format_args!("{}", s)),
                None => panic!("is_integer() but has no integer!"),
            };
        } else if self.is_list() {
            return match &self.list {
                Some(s) => f.write_fmt(format_args!("{:?}", s)),
                None => panic!("is_list() but has no list!"),
            };
        } else if self.is_dictionary() {
            return match &self.dictionary {
                Some(s) => f.write_fmt(format_args!("{:?}", s)),
                None => panic!("is_dictionary() but has no dictionary!"),
            };
        } else {
            return f.debug_struct("Result")
            .finish();
        };
    }
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

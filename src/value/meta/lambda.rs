use crate::parse::parse;
use crate::value::{ Value, ValRef, List };

#[derive(Debug, PartialEq)]
pub struct Lambda(ValRef);

impl Lambda {
    pub fn new(e: ValRef) -> Lambda {
        Lambda(e)
    }

    pub fn args(&self) -> Vec<String> {
        List::iter(&List::car(&List::cdr(&self.0)))
            .filter(|v| v.as_symbol().is_some())
            .map(|v| v.as_symbol().unwrap().to_owned())
            .collect()
    }

    pub fn definition(&self) -> ValRef {
        List::car(&List::cdr(&List::cdr(&self.0)))
    }

    pub fn from(s: &str) -> ValRef {
        Value::lambda(Lambda(parse(s).unwrap()))
    }
}

impl std::fmt::Display for Lambda {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[Lambda]")
    }
}

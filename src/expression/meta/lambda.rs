use crate::parse::parse;
use super::*;

#[derive(Debug, PartialEq)]
pub struct Lambda(ValRef);

impl Lambda {
    pub fn new(e: &ValRef) -> Lambda {
        Lambda(ValRef::clone(e))
    }

    pub fn args(&self) -> Vec<String> {
        List::iter(&List::car(&self.0))
            .filter(|e| e.is_atom())
            .filter(|e| e.as_atom().unwrap().is_symbol())
            .map(|e| e.as_atom().unwrap().as_symbol().unwrap().to_owned())
            .collect()
    }

    pub fn definition(&self) -> ValRef {
        List::car(&List::cdr(&self.0))
    }

    pub fn from(s: &str) -> ValRef {
        ValRef::lambda(Lambda(parse(s).unwrap()))
    }
}

impl std::fmt::Display for Lambda {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[Lambda]")
    }
}

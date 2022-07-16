use crate::parse::parse;
use super::*;

#[derive(Debug, PartialEq)]
pub struct Lambda(ExRef);

impl Lambda {
    pub fn new(e: &ExRef) -> Lambda {
        Lambda(ExRef::clone(e))
    }

    pub fn args(&self) -> Vec<String> {
        List::iter(&List::car(&self.0))
            .filter(|e| e.is_atom())
            .filter(|e| e.as_atom().unwrap().is_symbol())
            .map(|e| e.as_atom().unwrap().as_symbol().unwrap().to_owned())
            .collect()
    }

    pub fn definition(&self) -> ExRef {
        List::car(&List::cdr(&self.0))
    }

    pub fn from(s: &str) -> ExRef {
        ExRef::lambda(Lambda(parse(s).unwrap()))
    }
}

impl std::fmt::Display for Lambda {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[Lambda]")
    }
}

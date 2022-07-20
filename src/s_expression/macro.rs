use crate::parse::parse;
use crate::s_expression::{ SExpressionRef as SXRef, List };

#[derive(Debug, PartialEq)]
pub struct Macro(SXRef);

impl Macro {
    pub fn new(v: SXRef) -> Macro {
        Macro(v)
    }

    pub fn args(&self) -> Vec<String> {
        List::iter(&List::car(&List::cdr(&self.0)))
            .map(|v| v.as_symbol().unwrap().to_owned())
            .collect()
    }

    pub fn definition(&self) -> SXRef {
        List::car(&List::cdr(&List::cdr(&self.0)))
    }

    pub fn from(s: &str) -> SXRef {
        SXRef::r#macro(Macro(parse(s).unwrap()))
    }
}

impl std::fmt::Display for Macro {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[Macro]")
    }
}

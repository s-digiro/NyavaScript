use crate::parse::parse;
use crate::s_expression::{ SExpression, SExpressionRef, List };

#[derive(Debug, PartialEq)]
pub struct Lambda(SExpressionRef);

impl Lambda {
    pub fn new(e: SExpressionRef) -> Lambda {
        Lambda(e)
    }

    pub fn args(&self) -> Vec<String> {
        List::iter(&List::car(&List::cdr(&self.0)))
            .filter(|v| v.as_symbol().is_some())
            .map(|v| v.as_symbol().unwrap().to_owned())
            .collect()
    }

    pub fn definition(&self) -> SExpressionRef {
        List::car(&List::cdr(&List::cdr(&self.0)))
    }

    pub fn from(s: &str) -> SExpressionRef {
        SExpression::lambda(Lambda(parse(s).unwrap()))
    }
}

impl std::fmt::Display for Lambda {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[Lambda]")
    }
}

use crate::parse::{ parse, ParseError };
use crate::s_expression::List;
use std::convert::TryFrom;
use super::SExpressionRef as SXRef;

#[derive(Debug, PartialEq)]
pub struct Function {
    args: Vec<String>,
    definition: SXRef,
}

impl Function {
    pub fn new(sx: SXRef) -> Function {
        let args = List::iter(&List::car(&List::cdr(&sx)))
            .filter_map(|sx| sx.as_symbol().map(|s| s.into()))
            .collect();

        let definition = List::car(&List::cdr(&List::cdr(&sx)));

        Function { args, definition }
    }

    pub fn args(&self) -> &Vec<String> {
        &self.args
    }

    pub fn definition(&self) -> SXRef {
        SXRef::clone(&self.definition)
    }

    pub fn sxref(self) -> SXRef {
        SXRef::function(self)
    }
}

impl TryFrom<&str> for Function {
    type Error = ParseError;

    fn try_from(text: &str) -> Result<Self, Self::Error> {
        let ast = parse(text)?;
        let function = Function::new(ast);

        Ok(function)
    }
}

impl std::fmt::Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[Function]")
    }
}

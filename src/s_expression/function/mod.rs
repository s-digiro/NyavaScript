#[cfg(test)]
mod test;

use crate::parse::{ parse, ParseError };
use crate::s_expression::util;
use std::convert::TryFrom;
use super::SExpressionRef as SXRef;

#[derive(Clone, Debug, PartialEq)]
pub struct Function {
    args: Vec<String>,
    definition: SXRef,
}

impl Function {
    pub fn new(args: Vec<String>, definition: SXRef) -> Function {
        Function { args, definition }
    }

    pub fn args(&self) -> &Vec<String> {
        &self.args
    }

    pub fn definition(&self) -> SXRef {
        SXRef::clone(&self.definition)
    }
}

impl TryFrom<&str> for Function {
    type Error = ParseError;

    fn try_from(text: &str) -> Result<Self, Self::Error> {
        let ast = parse(text)?;

        Ok(ast.into())
    }
}

impl From<SXRef> for Function {
    fn from(sx: SXRef) -> Self {
        let args = util::car(&util::cdr(&sx)).iter()
            .filter_map(|sx| sx.as_symbol().map(|s| s.into()))
            .collect();

        let definition = util::car(&util::cdr(&util::cdr(&sx)));

        Function { args, definition }
    }
}

impl std::fmt::Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[Function]")
    }
}

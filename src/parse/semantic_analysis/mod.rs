mod error;
pub use error::*;

#[cfg(test)]
mod test;

use crate::s_expression::SExpressionRef as SXRef;
use super::syntactic_analysis::Syntax;

pub fn parse(tree: Syntax) -> Result<SXRef, SemanticError> {
    match tree {
        Syntax::List(l) => parse_list(l),
        Syntax::Number(n) => Ok(SXRef::number(n)),
        Syntax::String(s) => Ok(SXRef::string(s)),
        Syntax::Symbol(s) => Ok(SXRef::symbol(s)),
        Syntax::Dot(_, _) => todo!(),
    }
}

fn parse_list(list: Vec<Syntax>) -> Result<SXRef, SemanticError> {
    if list.len() == 0 {
        Ok(SXRef::nil())
    } else {
        let children = list.into_iter()
            .map(|syn| parse(syn))
            .collect::<Result<Vec<SXRef>, SemanticError>>()?;

        Ok(SXRef::from(children))
    }
}

#[cfg(test)]
mod test;

use crate::s_expression::SExpressionRef as SXRef;
use super::syntactic_analysis::Syntax;

pub fn parse(tree: Syntax) -> SXRef {
    match tree {
        Syntax::List(l) => parse_list(l),
        Syntax::Number(n) => SXRef::number(n),
        Syntax::String(s) => SXRef::string(s),
        Syntax::Symbol(s) => SXRef::symbol(s),
    }
}

fn parse_list(list: Vec<Syntax>) -> SXRef {
    if list.len() == 0 {
        SXRef::nil()
    } else {
        let children = list.into_iter()
            .map(|syn| parse(syn))
            .collect::<Vec<SXRef>>();

        SXRef::from(children)
    }
}

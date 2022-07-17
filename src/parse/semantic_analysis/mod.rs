#[cfg(test)]
mod test;

use crate::expression::{ Value, ValRef, List };
use super::syntactic_analysis::Syntax;

pub fn parse(tree: Syntax) -> ValRef {
    match tree {
        Syntax::List(l) => parse_list(l),
        Syntax::Number(n) => Value::number(n),
        Syntax::String(s) => Value::string(s),
        Syntax::Symbol(s) => Value::symbol(s),
    }
}

fn parse_list(list: Vec<Syntax>) -> ValRef {
    if list.len() == 0 {
        List::nil()
    } else {
        let children = list.into_iter()
            .map(|syn| parse(syn))
            .collect::<Vec<ValRef>>();

        List::from(children)
    }
}

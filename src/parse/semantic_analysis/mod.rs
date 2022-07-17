#[cfg(test)]
mod test;

use crate::expression::{ Atom, ExRef, List };
use super::syntactic_analysis::Syntax;

pub fn parse(tree: Syntax) -> ExRef {
    match tree {
        Syntax::List(l) => parse_list(l),
        Syntax::Number(n) => parse_number(n),
        Syntax::String(s) => parse_string(s),
        Syntax::Symbol(a) => parse_symbol(a),
    }
}

fn parse_symbol(symbol: String) -> ExRef {
    Atom::symbol(&symbol)
}

fn parse_list(list: Vec<Syntax>) -> ExRef {
    if list.len() == 0 {
        List::nil()
    } else {
        let children = list.into_iter()
            .map(|syn| parse(syn))
            .collect::<Vec<ExRef>>();

        List::from(children)
    }
}

fn parse_string(s: String) -> ExRef {
    Atom::string(&s)
}

fn parse_number(n: isize) -> ExRef {
    Atom::number(n)
}

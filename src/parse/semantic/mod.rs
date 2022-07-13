#[cfg(test)]
mod test;

mod special;
use special::Special;

use crate::parse::{ Expression as Expr, Syntax };

pub fn parse(tree: Syntax) -> Result<Expr, String> {
    match tree {
        Syntax::Atom(a) => parse_atom(a),
        Syntax::List(l) => parse_list(l),
        Syntax::Number(n) => parse_number(n),
        Syntax::String(s) => parse_string(s),
    }
}

fn parse_atom(atom: String) -> Result<Expr, String> {
    Ok(Expr::Atom(atom))
}

fn parse_list(mut list: Vec<Syntax>) -> Result<Expr, String> {
    if list.len() == 0 {
        return Ok(Expr::list())
    }

    if list[0].is_atom() {
        if let Ok(special) = Special::try_from(list[0].as_atom().unwrap()) {
            list.remove(0);

            return special.parse(list)
        }
    }

    let children = list.into_iter()
        .map(|syn| parse(syn))
        .collect::<Result<Vec<Expr>, String>>()?;

    Ok(Expr::List(children))
}

fn parse_string(s: String) -> Result<Expr, String> {
    Ok(Expr::String(s))
}

fn parse_number(n: isize) -> Result<Expr, String> {
    Ok(Expr::Number(n))
}

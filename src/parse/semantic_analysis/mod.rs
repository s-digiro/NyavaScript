mod error;
pub use error::*;

#[cfg(test)]
mod test;

use crate::s_expression::{
    SExpressionRef as SXRef,
    ConsCell,
    util,
};

use super::syntactic_analysis::Syntax;

pub fn parse(tree: Syntax) -> Result<SXRef, SemanticError> {
    match tree {
        Syntax::List(l) => parse_list(l),
        Syntax::Number(n) => Ok(SXRef::number(n)),
        Syntax::String(s) => Ok(SXRef::string(s)),
        Syntax::Symbol(s) => Ok(SXRef::symbol(s)),
        Syntax::Dot(_, _) => panic!("This should never happen"),
    }
}

fn parse_list(list: Vec<Syntax>) -> Result<SXRef, SemanticError> {
    if list.len() == 0 {
        Ok(SXRef::nil())
    } else {
        let dot_status = list_has_dot_in_proper_position(&list);

        match dot_status {
            DotStatus::Malformed => Err(SemanticError::DotSyntaxNotAtListEnd),
            DotStatus::NoDot => {
                let children = list.into_iter()
                    .map(|syn| parse(syn))
                    .collect::<Result<Vec<SXRef>, SemanticError>>()?;

                Ok(SXRef::from(children))
            },
            DotStatus::Okay => {
                let mut it = list.into_iter().rev();

                let (car, cdr) = if let Syntax::Dot(car, cdr) = it.next().unwrap() {
                    (car, cdr)
                } else {
                    panic!("This should never happen");
                };

                let car = match car {
                    Some(car) => parse(*car)?,
                    None => SXRef::nil(),
                };

                let cdr = match cdr {
                    Some(cdr) => parse(*cdr)?,
                    None => SXRef::nil(),
                };

                let mut current = SXRef::cons_cell(ConsCell::new(
                    car,
                    cdr,
                ));

                for syn in it {
                    let sx = parse(syn)?;

                    current = util::cons(&sx, &current);
                }

                Ok(current)
            }
        }
    }
}

enum DotStatus {
    Malformed,
    NoDot,
    Okay,
}

fn list_has_dot_in_proper_position(list: &Vec<Syntax>) -> DotStatus {
    let mut has_dot = false;

    for syn in list {
        if has_dot {
            return DotStatus::Malformed;
        }

        match syn {
            Syntax::Dot(..) => has_dot = true,
            _ => (),
        }
    }

    if has_dot {
        DotStatus::Okay
    } else {
        DotStatus::NoDot
    }
}

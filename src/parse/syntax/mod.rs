#[cfg(test)]
mod test;

mod errors;
pub use errors::*;

use std::mem;

use crate::parse::data::{ Symbol, Syntax };

pub fn parse(symbols: Vec<Symbol>) -> Result<Syntax, Error> {
    let mut stack = Vec::new();
    let mut current = Syntax::List(Vec::new());

    for sym in symbols.into_iter() {
        match sym {
            Symbol::OpenList => stack.push(
                mem::replace(&mut current, Syntax::list())
            ),
            Symbol::CloseList => {
                let child = mem::replace(
                    &mut current,
                    stack.pop().ok_or(Error::UnmatchedCloseListError)?
                );

                current.as_mut_list().unwrap().push(child);
            },
            Symbol::Atom(a) => current.as_mut_list().unwrap()
                .push(Syntax::Atom(a)),
            Symbol::Quote => current.as_mut_list().unwrap()
                .push(Syntax::atom("quote")),
            Symbol::String(s) => current.as_mut_list().unwrap()
                .push(Syntax::String(s)),
            Symbol::Number(num) => current.as_mut_list().unwrap()
                .push(Syntax::Number(num)),
        }
    }

    if !stack.is_empty() {
        return Err(Error::UnmatchedOpenListError)
    }

    Ok(current)
}
